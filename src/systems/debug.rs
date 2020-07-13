use crate::components::*;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

pub struct DebugSystem;

impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, EditorRootTag>,
        ReadStorage<'s, Steering>,
        ReadStorage<'s, PlayerTag>,
        ReadStorage<'s, DebugPosGhostTag>,
        ReadStorage<'s, DebugSteeringGhostTag>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            root_tags,
            steerings,
            player_tags,
            pos_ghost_tags,
            steering_ghost_tags,
        ): Self::SystemData,
    ) {
        // Sets the transform on the ghost tags.
        // This is a debug thing to show us where the player is going.
        let maybe_steering = (&player_tags, &steerings)
            .join()
            .map(|(_, steering)| steering)
            .next();
        if let Some(steering) = maybe_steering {
            for (_, transform) in (&steering_ghost_tags, &mut transforms).join() {
                let (centered_x, centered_y) = steering.to_centered_coords(steering.destination);
                transform.set_translation_x(centered_x);
                transform.set_translation_y(centered_y);
            }
            for (_, transform) in (&pos_ghost_tags, &mut transforms).join() {
                transform.set_translation_x(steering.pos.x as f32 + 0.5);
                transform.set_translation_y(steering.pos.y as f32 + 0.5);
            }
        }
    }
}
