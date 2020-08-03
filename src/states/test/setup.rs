use std::path::{Path, PathBuf};

use amethyst::core::math::Vector3;
use amethyst::prelude::WorldExt;
use amethyst::ui::UiPrefab;
use amethyst::State;
use amethyst::StateEvent;
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    assets::{AssetStorage, Handle, Loader, Prefab},
    core::{timing::Time, transform::Transform, Parent},
    ecs::{prelude::World, Entities, Entity, Join, ReadStorage, WriteStorage},
    input::{get_key, is_close_requested, is_key_down, InputEvent, VirtualKeyCode},
    prelude::*,
    renderer::{
        formats::texture::ImageFormat, palette::Srgba, resources::Tint, sprite::SpriteRender,
        Camera, SpriteSheet, Texture,
    },
    utils::application_root_dir,
    window::{MonitorIdent, ScreenDimensions, Window},
    winit::{Event, WindowEvent},
    StateData, Trans,
};

use precompile::AnimationId;

use crate::components::*;
use crate::entities::*;
use crate::game_data::CustomGameData;
use crate::levels::*;
use crate::resources::*;
use crate::states::{window_event_handler, EditorState, PausedState};

pub fn setup_test(test: MovementTest, world: &mut World) {
    clear_previous_test(world);
    load_level_from_file(&test, world);
}

fn clear_previous_test(world: &mut World) {
    world.exec(
        |(entities, tags): (Entities, ReadStorage<MovementTestScopeTag>)| {
            for (entity, _) in (&entities, &tags).join() {
                entities.delete(entity);
            }
        },
    );
}

fn load_level_from_file(test: &MovementTest, world: &mut World) {
    let file_name = match test {
        MovementTest::Jump2Wide => "jump_2_wide.ron",
        MovementTest::Jump4Wide => "jump_4_wide.ron",
    };
    let level_file = application_root_dir()
        .expect("Root dir not found!")
        .join("assets/")
        .join("tests/")
        .join(file_name);
    load_level(&level_file, world);
}
