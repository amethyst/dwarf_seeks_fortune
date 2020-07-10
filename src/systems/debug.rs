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
        ReadStorage<'s, Pos>,
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
            positions,
            root_tags,
            steerings,
            player_tags,
            pos_ghost_tags,
            steering_ghost_tags,
        ): Self::SystemData,
    ) {
        // Sets the transform on the ghost tags.
        // This is a debug thing to show us where the player is going.
        let maybe_destination = (&player_tags, &positions, &steerings)
            .join()
            .map(|(_, pos, steering)| (pos, steering.destination))
            .nth(0);
        if let Some((pos, destination)) = maybe_destination {
            for (_, transform) in (&steering_ghost_tags, &mut transforms).join() {
                transform.set_translation_x(destination.x as f32 + 1.0);
                transform.set_translation_y(destination.y as f32 + 1.0);
            }
            for (_, transform) in (&pos_ghost_tags, &mut transforms).join() {
                transform.set_translation_x(pos.x as f32 + 0.5);
                transform.set_translation_y(pos.y as f32 + 0.5);
            }
        }
    }
}
