use amethyst::prelude::WorldExt;
use amethyst::ui::UiPrefab;
use amethyst::State;
use amethyst::StateEvent;
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    assets::{AssetStorage, Handle, Loader, Prefab},
    core::{
        math::{Point2, Vector3},
        transform::Transform,
        Parent,
    },
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
use crate::levels::map::*;
use crate::resources::*;
use crate::states::PausedState;

pub fn load_level(world: &mut World) {
    let level_file = application_root_dir()
        .expect("Root dir not found!")
        .join("assets/")
        .join("tiles/")
        .join("testlevel.ron");
    let map = Map::load(level_file);
    // println!("Map loaded: {:?}", map);
    //TODO: handle map loading error gracefully.
    map.expect("Failed to load map.")
        .tiles
        .iter()
        .for_each(|tile| {
            // println!("Tile: {:?}", tile);
            let still_asset = load_still_asset(tile, &world);
            let anim_asset = load_anim_asset(tile, &world);
            let transform =
                load_transform(&tile.pos, &tile.tile_type.dimens, &tile.tile_type.asset);
            let mut builder = world.create_entity();
            if let Some(still_asset) = still_asset {
                builder = builder.with(still_asset);
            }
            if let Some(anim_asset) = anim_asset {
                builder = builder.with(anim_asset);
            }
            builder = builder.with(transform);
            match tile.tile_type.entity_type {
                EntityType::Player => {
                    let player = build_player(builder);
                    build_frames(player, world);
                }
                _ => {
                    builder.build();
                }
            };
        });
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
        &Point2::new(0, 0),
        &Point2::new(2, 2),
        &AssetType::Still(SpriteType::Frame, 0),
    );
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(steering_ghost_transform)
        .with(DebugSteeringGhostTag)
        .build();
    let pos_ghost_transform = load_transform(
        &Point2::new(0, 0),
        &Point2::new(1, 1),
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
        .with(DiscretePos::default())
        .with(Velocity::default())
        .with(Steering::default())
        .with(PlayerTag)
        .build()
}

pub(crate) fn load_transform(
    pos: &Point2<i32>,
    dimens: &Point2<i32>,
    asset: &AssetType,
) -> Transform {
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

fn load_still_asset(tile: &Tile, world: &World) -> Option<SpriteRender> {
    match &tile.tile_type.asset {
        AssetType::Animated(anim) => None,
        AssetType::Still(spritesheet, sprite_nr) => {
            let handle = world.read_resource::<Assets>().get_still(&spritesheet);
            Some(SpriteRender {
                sprite_sheet: handle,
                sprite_number: *sprite_nr,
            })
        }
    }
}

fn load_anim_asset(tile: &Tile, world: &World) -> Option<Handle<Prefab<MyPrefabData>>> {
    match &tile.tile_type.asset {
        AssetType::Animated(anim) => {
            let handle = world.read_resource::<Assets>().get_animated(&anim);
            Some(handle)
        }
        AssetType::Still(spritesheet, sprite_nr) => None,
    }
}
