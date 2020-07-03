use crate::components::*;
use crate::resources::*;
use amethyst::{
    core::transform::Transform,
    core::{timing::Time, Parent},
    ecs::{
        prelude::{Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
        Entities,
    },
    input::{InputHandler, StringBindings},
    renderer::{sprite::SpriteRender, Camera},
    window::ScreenDimensions,
};

/// This system updates the camera frame position to the player's position.
pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        ReadStorage<'s, PlayerTag>,
        ReadStorage<'s, CameraFrameTag>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (players, camera_frames, mut transforms): Self::SystemData) {
        let maybe_player_pos = (&players, &transforms)
            .join()
            .map(|(_, transform)| (transform.translation().x, transform.translation().y))
            .nth(0);
        if let Some((player_x, player_y)) = maybe_player_pos {
            for (_, transform) in (&camera_frames, &mut transforms).join() {
                transform.set_translation_x(player_x);
                transform.set_translation_y(player_y);
            }
        }
    }
}

/// Recreates the camera if the window is resized.
#[derive(Default)]
pub struct ResizeSystem;

impl<'s> System<'s> for ResizeSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Parent>,
        ReadStorage<'s, CameraFrameTag>,
        ReadExpect<'s, ScreenDimensions>,
        Write<'s, ResizeState>,
    );

    fn run(
        &mut self,
        (entities, mut cameras, mut transforms, mut parents, camera_frames, dimens, mut resize): Self::SystemData,
    ) {
        if *resize != ResizeState::Resizing {
            return;
        }
        println!(
            "Resizing camera. Camera entity will be destroyed and recreated. ({:?}, {:?})",
            dimens.width(),
            dimens.height()
        );
        let frame = (&*entities, &camera_frames)
            .join()
            .map(|(entity, _)| entity)
            .nth(0);
        let cam = (&*entities, &cameras)
            .join()
            .map(|(entity, _)| entity)
            .nth(0);
        if let Some(frame) = frame {
            if let Some(cam) = cam {
                entities.delete(cam);
            }
            entities
                .build_entity()
                .with(Parent { entity: frame }, &mut parents)
                .with(
                    Camera::standard_2d(dimens.width(), dimens.height()),
                    &mut cameras,
                )
                .with(Transform::default(), &mut transforms)
                .build();
        }
        *resize = ResizeState::Idle;
    }
}
