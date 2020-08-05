use amethyst::prelude::WorldExt;
use amethyst::ui::UiPrefab;
use amethyst::State;
use amethyst::StateEvent;
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    assets::{AssetStorage, Handle, Loader, Prefab},
    config::ConfigError,
    core::{math::Vector3, transform::Transform, Parent},
    ecs::{prelude::World, Entities, Entity, EntityBuilder, Join, ReadStorage, WriteStorage},
    input::{get_key, is_close_requested, is_key_down, InputEvent, VirtualKeyCode},
    prelude::*,
    renderer::{
        formats::texture::ImageFormat, palette::Srgba, resources::Tint, sprite::SpriteRender,
        Camera, SpriteSheet, Texture, Transparent,
    },
    utils::application_root_dir,
    window::ScreenDimensions,
    winit::{Event, WindowEvent},
    StateData, Trans,
};
use precompile::{AnimationId, MyPrefabData};

use crate::components::*;
use crate::game_data::CustomGameData;
use crate::levels::{Archetype, DepthLayer, Level, TileDefinition, TileDefinitions};
use crate::resources::*;
use crate::states::PausedState;
use std::error::Error;
use std::path::PathBuf;

pub fn load_tile_definitions() -> Result<TileDefinitions, ConfigError> {
    let file = application_root_dir()
        .expect("Root dir not found!")
        .join("assets/")
        .join("tiles/")
        .join("tiles.ron");
    TileDefinitions::load(file)
}

pub fn load_level(level_file: &PathBuf, world: &mut World) -> Result<(), ConfigError> {
    let mut win_condition = WinCondition::default();
    let display_debug_frames = world.read_resource::<DebugConfig>().display_debug_frames;
    let fallback_def = TileDefinition::fallback();
    let tile_defs = load_tile_definitions()?;
    let level = Level::load(level_file)?;
    add_background(world, &level.pos, &level.dimens);
    level.tiles.iter().for_each(|(pos, tile_def_key)| {
        let tile_def = tile_defs.get(tile_def_key);
        let still_asset = load_still_asset(tile_def, &world.read_resource::<Assets>());
        let anim_asset = load_anim_asset(tile_def, &world.read_resource::<Assets>());
        let transform = if let Some(asset) = &tile_def.asset {
            Some(load_transform(
                &pos,
                &tile_def.depth,
                &tile_def.dimens,
                asset,
            ))
        } else {
            None
        };
        let mut builder = world.create_entity();
        if let Some(still_asset) = still_asset {
            builder = builder.with(still_asset);
        }
        if let Some(anim_asset) = anim_asset {
            builder = builder.with(anim_asset);
        }
        if let Some(transform) = transform {
            builder = builder.with(transform);
        }
        builder = builder.with(Block { pos: pos.clone() });
        match tile_def.archetype {
            Archetype::Player => {
                let player = build_player(builder, pos, tile_def);
                if display_debug_frames {
                    build_frames(player, world, tile_def);
                }
            }
            Archetype::Key => {
                win_condition.add_key(pos);
                builder.with(Key::new(pos.clone())).build();
            }
            Archetype::Tool(tool_type) => {
                if let Some(AssetType::Still(sprite, sprite_nr)) = tile_def.asset {
                    builder
                        .with(Tool::new(tool_type, sprite, sprite_nr))
                        .build();
                } else {
                    error!(
                        "Tool definition {:?} did not have still asset.",
                        tile_def_key
                    );
                }
            }
            Archetype::Door => {
                builder.with(ExitDoor).build();
            }
            _ => {
                builder.build();
            }
        };
    });
    add_key_displays_to_door(world, &win_condition);
    world.insert(win_condition);
    world.insert(TileMap::new(level, tile_defs));
    Ok(())
}

fn add_background(world: &mut World, pos: &Pos, dimens: &Pos) {
    let transform = load_transform(
        pos,
        &DepthLayer::Background,
        dimens,
        &AssetType::Still(SpriteType::Selection, 1),
    );
    let asset = load_asset_from_world(&SpriteType::Selection, 1, world);
    world.create_entity().with(transform).with(asset).build();
}

fn add_key_displays_to_door(world: &mut World, win_condition: &WinCondition) {
    let door_entity = world.exec(|(doors, entities): (ReadStorage<ExitDoor>, Entities)| {
        (&doors, &entities)
            .join()
            .map(|(_, entity)| (entity))
            .next()
    });
    if let Some((door_entity)) = door_entity {
        win_condition
            .keys
            .iter()
            .enumerate()
            .for_each(|(index, key)| {
                let mut transform = Transform::default();
                let x_offset = (index % 4);
                let y_offset = (index / 4);
                transform.set_translation_x((-1. + x_offset as f32) * 64.);
                transform.set_translation_y((1. + y_offset as f32) * 64.);
                transform.set_translation_z(1.); //One higher than parent.
                transform.set_scale(Vector3::new(0.5, 0.5, 1.0));
                let sprite = load_asset_from_world(&SpriteType::Blocks, 3, world);
                world
                    .create_entity()
                    .with(Parent {
                        //TODO:FIXME: don't make this a parent, leads to problems.
                        entity: door_entity,
                    })
                    .with(transform)
                    .with(sprite)
                    .with(KeyDisplay::new(key.clone()))
                    .build();
            });
    }
}

fn build_frames(player: Entity, world: &mut World, tile_def: &TileDefinition) {
    let frame = world
        .read_resource::<Assets>()
        .get_still(&SpriteType::Frame);
    let sprite_render = SpriteRender {
        sprite_sheet: frame,
        sprite_number: 0, // First sprite
    };

    let steering_ghost_transform = load_transform(
        &Pos::default(),
        &DepthLayer::UiElements,
        &tile_def.dimens,
        &AssetType::Still(SpriteType::Frame, 0),
    );
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(steering_ghost_transform)
        .with(DebugSteeringGhostTag)
        .build();
    let pos_ghost_transform = load_transform(
        &Pos::default(),
        &DepthLayer::UiElements,
        &Pos::new(1, 1),
        &AssetType::Still(SpriteType::Frame, 0),
    );
    world
        .create_entity()
        .with(sprite_render)
        .with(pos_ghost_transform)
        .with(DebugPosGhostTag)
        .build();
}

fn build_player(builder: EntityBuilder, pos: &Pos, tile_def: &TileDefinition) -> Entity {
    builder
        .with(Transparent)
        .with(Velocity::default())
        .with(SteeringIntent::default())
        .with(Steering::new(pos.clone(), tile_def.dimens.clone()))
        .with(Player::default())
        .build()
}

pub fn load_transform(pos: &Pos, depth: &DepthLayer, dimens: &Pos, asset: &AssetType) -> Transform {
    let asset_dimensions = get_asset_dimensions(&asset);
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        pos.x as f32 + dimens.x as f32 * 0.5,
        pos.y as f32 + dimens.y as f32 * 0.5,
        depth.z(),
    );
    transform.set_scale(Vector3::new(
        dimens.x as f32 / asset_dimensions.x as f32,
        dimens.y as f32 / asset_dimensions.y as f32,
        1.0,
    ));
    transform
}

pub fn load_still_asset(tile: &TileDefinition, assets: &Assets) -> Option<SpriteRender> {
    match &tile.asset? {
        AssetType::Animated(anim) => None,
        AssetType::Still(spritesheet, sprite_nr) => {
            let handle = assets.get_still(&spritesheet);
            Some(SpriteRender {
                sprite_sheet: handle,
                sprite_number: *sprite_nr,
            })
        }
    }
}

pub fn load_asset_from_world(
    sprite: &SpriteType,
    sprite_nr: usize,
    world: &mut World,
) -> SpriteRender {
    let assets = world.read_resource::<Assets>();
    SpriteRender {
        sprite_sheet: assets.get_still(sprite),
        sprite_number: sprite_nr,
    }
}

pub fn load_anim_asset(
    tile: &TileDefinition,
    assets: &Assets,
) -> Option<Handle<Prefab<MyPrefabData>>> {
    match &tile.asset? {
        AssetType::Animated(anim) => {
            let handle = assets.get_animated(&anim);
            Some(handle)
        }
        AssetType::Still(spritesheet, sprite_nr) => None,
    }
}
