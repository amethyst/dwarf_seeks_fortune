use crate::components::*;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut transforms, velocities, time): Self::SystemData,
    ) {
        for (transform, velocity) in (&mut transforms, &velocities).join() {
            transform.set_translation_x(
                (transform.translation().x + time.delta_seconds() * velocity.x),
            );
            transform.set_translation_y(
                (transform.translation().y + time.delta_seconds() * velocity.y),
            );
        }
    }
}

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, DiscretePos>,
        WriteStorage<'s, Steering>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, PlayerTag>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (mut transforms, mut discrete_positions, mut steerings, mut velocities, player_tags, input): Self::SystemData,
    ) {
        for (_, transform, discrete_pos, steering, velocity) in (&player_tags, &transforms, &mut discrete_positions, &mut steerings, &mut velocities).join() {
            //TODO:
            // 1: Set current discrete position.
            // 2: Set steering based on user input.
            // 3: Set velocity based on current position and desired position.
            // 4: If necessary, adjust position, snap to grid.

            let actual_pos_x = transform.translation().x as i32;
            let base_pos = (actual_pos_x - 50).div_euclid(50);// -50 because pos is off by 1 block (player width / 2)
            let pos_remainder = actual_pos_x.rem_euclid(50);
            // let pos_remainder = transform.translation().x % 50.0;
            discrete_pos.x = if pos_remainder > 25 {
                base_pos + 1
            } else {
                base_pos
            };
            println!("actual_pos={:?} base_pos={:?} remainder: {:?} discrete: {:?}", actual_pos_x, base_pos, pos_remainder, discrete_pos.x);

            let x_axis = input.axis_value("move_x");
            // let y_axis = input.axis_value("move_y");
            if let Some(signum) = x_axis {
                if signum.abs() > f32::EPSILON {
                    steering.direction = signum;
                    steering.destination.x = discrete_pos.x + signum as i32;
                }
            }

            //=========================================

            let desired_pos = steering.destination.x as f32 * 50.0 + 50.0;
            let delta = desired_pos - transform.translation().x;
            let delta_signum =
                if delta.abs() < f32::EPSILON {
                    0.0
                } else {
                    delta.signum()
                };
            if (delta_signum * steering.direction).is_sign_positive() {
                velocity.x = delta_signum * 300.0;
            } else {
                velocity.x = 0.0;
            }
        }
    }
}
