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
        Camera, SpriteSheet, Texture,
    },
    utils::application_root_dir,
    window::ScreenDimensions,
    winit::{Event, WindowEvent},
    StateData, Trans,
};
use precompile::{AnimationId, MyPrefabData};

use crate::components::*;
use crate::game_data::CustomGameData;
use crate::levels::{Archetype, Level, TileDefinition, TileDefinitions};
use crate::resources::*;
use crate::states::PausedState;
use std::error::Error;

pub fn load_tile_definitions() -> Result<TileDefinitions, ConfigError> {
    let file = application_root_dir()
        .expect("Root dir not found!")
        .join("assets/")
        .join("tiles/")
        .join("tiles.ron");
    let tile_defs = TileDefinitions::load(file);
    // println!("Tile definitions loaded: {:?}", tile_defs);
    tile_defs
}

pub fn load_level(world: &mut World) -> Result<(), ConfigError> {
    let fallback_def = TileDefinition::fallback();
    let tile_defs = load_tile_definitions()?;
    let level_file = application_root_dir()
        .expect("Root dir not found!")
        .join("assets/")
        .join("levels/")
        .join("generated.ron");
    let level = Level::load(level_file)?;
    // println!("Map loaded: {:?}", map);
    level.tile_defs.iter().for_each(|(pos, tile_def_key)| {
        let tile_def = tile_defs.get(tile_def_key);
        let still_asset = load_still_asset(tile_def, &world.read_resource::<Assets>());
        let anim_asset = load_anim_asset(tile_def, &world.read_resource::<Assets>());
        let transform = if let Some(asset) = &tile_def.asset {
            Some(load_transform(&pos, &tile_def.dimens, asset))
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
        match tile_def.archetype {
            Archetype::Player => {
                let player = build_player(builder);
                build_frames(player, world);
            }
            _ => {
                builder.build();
            }
        };
    });
    world.insert(TileMap::new(level, tile_defs));
    Ok(())
}

fn build_frames(player: Entity, world: &mut World) {
    let frame = world
        .read_resource::<Assets>()
        .get_still(&SpriteType::Frame);
    let sprite_render = SpriteRender {
        sprite_sheet: frame,
        sprite_number: 0, // First sprite
    };

    let steering_ghost_transform = load_transform(
        &Pos::new(0, 0),
        &Pos::new(2, 2),
        &AssetType::Still(SpriteType::Frame, 0),
    );
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(steering_ghost_transform)
        .with(DebugSteeringGhostTag)
        .build();
    let pos_ghost_transform = load_transform(
        &Pos::new(0, 0),
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

fn build_player(builder: EntityBuilder) -> Entity {
    builder
        .with(Pos::default())
        .with(Velocity::default())
        .with(Steering::default())
        .with(PlayerTag)
        .build()
}

pub fn load_transform(pos: &Pos, dimens: &Pos, asset: &AssetType) -> Transform {
    let asset_dimensions = get_asset_dimensions(&asset);
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        pos.x as f32 + dimens.x as f32 * 0.5,
        pos.y as f32 + dimens.y as f32 * 0.5,
        0.0,
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
