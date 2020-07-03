use crate::components::*;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
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
        let maybe_player_pos = (&players, &transforms).join()
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
