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
    ecs::{prelude::World, Entities, Entity, Join, ReadStorage, Write, WriteStorage},
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
use crate::states::{EditorState, PausedState};
use amethyst::core::ecs::ReadExpect;

/// Handle some general behaviour related to the window that should be executed in any State.
pub(crate) fn handle(event: &StateEvent, world: &mut World) {
    match event {
        StateEvent::Input(InputEvent::ActionPressed(action)) => {
            if action == "toggleFullscreen" {
                toggle_fullscreen(world);
            }
        }
        StateEvent::Window(Event::WindowEvent {
            window_id: _,
            event: WindowEvent::Resized(_),
        }) => {
            resize_camera(world);
        }
        _ => (),
    };
}

/// Toggles fullscreen mode.
fn toggle_fullscreen(world: &mut World) {
    let window = world.fetch_mut::<Window>();
    window.set_fullscreen(if window.get_fullscreen().is_none() {
        Some(window.get_current_monitor())
    } else {
        None
    });
}

/// Responds to window resize events. Recreates the camera with the new dimensions.
fn resize_camera(world: &mut World) {
    world.exec(
        |mut data: (
            Entities,
            WriteStorage<Camera>,
            WriteStorage<Transform>,
            WriteStorage<Parent>,
            ReadStorage<CameraFrame>,
            ReadExpect<ScreenDimensions>,
        )| {
            let (entities, mut cameras, mut transforms, mut parents, camera_frames, screen_dimens) =
                data;
            let frame = (&*entities, &camera_frames)
                .join()
                .map(|(entity, _)| entity)
                .next();
            let cam = (&*entities, &cameras)
                .join()
                .map(|(entity, _)| entity)
                .next();
            if let Some(frame) = frame {
                if let Some(cam) = cam {
                    entities.delete(cam);
                }
                if screen_dimens.width() > f32::EPSILON && screen_dimens.height() > f32::EPSILON {
                    entities
                        .build_entity()
                        .with(Parent { entity: frame }, &mut parents)
                        .with(
                            Camera::standard_2d(screen_dimens.width(), screen_dimens.height()),
                            &mut cameras,
                        )
                        .with(Transform::default(), &mut transforms)
                        .build();
                }
            }
        },
    );
}
