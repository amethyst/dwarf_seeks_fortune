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
        ReadStorage<'s, DiscretePos>,
        ReadStorage<'s, Steering>,
        ReadStorage<'s, DebugOrbTag>,
        ReadStorage<'s, PlayerTag>,
        ReadStorage<'s, DebugPosGhostTag>,
        ReadStorage<'s, DebugSteeringGhostTag>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut transforms, positions, steerings, debug_orbs, player_tags, pos_ghost_tags, steering_ghost_tags, input, screen_dimens): Self::SystemData) {
        for (transform, debug_orb) in (&mut transforms, &debug_orbs).join() {
            let x_axis = input.axis_value("move_x");
            let y_axis = input.axis_value("move_y");
            if let Some(signum) = x_axis {
                if signum.abs() > 0.01 {
                    println!("Move x signum={:?}\t dimens:{:?}", signum, screen_dimens.width());
                    transform.set_translation_x((screen_dimens.width() * signum).max(0.0));
                }
            }
            if let Some(signum) = y_axis {
                if signum.abs() > 0.01 {
                    println!("Move y signum={:?}\t dimens:{:?}", signum, screen_dimens.height());
                    transform.set_translation_y((screen_dimens.height() * signum).max(0.0));
                }
            }
        }

        // Sets the transform on the ghost tags.
        // This is a debug thing to show us where the player is going.
        let maybe_destination = (&player_tags, &positions, &steerings).join()
            .map(|(_, pos, steering)| (pos, steering.destination))
            .nth(0);
        if let Some((pos, destination)) = maybe_destination {
            for (_, transform) in (&steering_ghost_tags, &mut transforms).join() {
                transform.set_translation_x(
                    (destination.x * 50 + 50) as f32,
                );
                transform.set_translation_y(
                    (destination.y * 50 + 50) as f32,
                );
            }
            for (_, transform) in (&pos_ghost_tags, &mut transforms).join() {
                transform.set_translation_x(
                    (pos.x * 50 + 25) as f32,
                );
                transform.set_translation_y(
                    (pos.y * 50 + 25) as f32,
                );
            }
        }
    }
}