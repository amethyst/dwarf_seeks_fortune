use crate::components::*;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, Write, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};
use crate::config::*;
use crate::resources::*;


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
        Read<'s, DebugConfig>,
        Write<'s, History>,
    );

    fn run(
        &mut self,
        (mut transforms, mut discrete_positions, mut steerings, mut velocities, player_tags, input, config, mut history): Self::SystemData,
    ) {
        for (_, transform, discrete_pos, steering, velocity) in (&player_tags, &mut transforms, &mut discrete_positions, &mut steerings, &mut velocities).join() {
            // 1: Set current discrete position.
            // 2: Set steering based on user input.
            // 3: Set velocity based on current position and desired position.
            // 4: If necessary, adjust position, snap to grid.

            let old_pos = discrete_pos.clone();
            discrete_pos.x = calc_discrete_pos_x(transform);
            if old_pos != *discrete_pos {
                history.add_frame(Frame::new(discrete_pos.clone()));
            }

            let input_x = input.axis_value("move_x").unwrap_or(0.0);
            if input_x.abs() > f32::EPSILON {
                steering.direction = input_x;
                let offset_from_discrete_pos = (discrete_pos.x * 50) as f32 - (transform.translation().x - 50.);
                if offset_from_discrete_pos < f32::EPSILON && input_x > f32::EPSILON {
                    steering.destination.x = discrete_pos.x + 1;
                } else if offset_from_discrete_pos > -f32::EPSILON && input_x < f32::EPSILON {
                    steering.destination.x = discrete_pos.x - 1;
                } else if ((steering.destination.x - discrete_pos.x) * input_x as i32).is_negative() {
                    steering.destination.x = discrete_pos.x;
                }
            }

            let desired_pos = steering.destination.x as f32 * 50.0 + 50.0;
            let delta = desired_pos - transform.translation().x;
            let delta_signum =
                if delta.abs() < f32::EPSILON {
                    0.0
                } else {
                    delta.signum()
                };
            if (delta_signum * steering.direction).is_sign_positive() {
                velocity.x = delta_signum * config.player_speed;
            } else {
                velocity.x = 0.0;
                transform.set_translation_x((discrete_pos.x * 50 + 50) as f32);
            }
        }
    }
}

fn calc_discrete_pos_x(transform: &Transform) -> i32 {
    let actual_pos_x = transform.translation().x as i32;
    let base_pos = (actual_pos_x - 50).div_euclid(50);
    let pos_remainder = actual_pos_x.rem_euclid(50);
    if pos_remainder > 25 {
        base_pos + 1
    } else {
        base_pos
    }
}
