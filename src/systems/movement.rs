use crate::components::*;
use crate::levels::*;
use crate::resources::*;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

/// For every entity with a velocity and a transform, updates the transform according to the
/// velocity.
pub struct VelocitySystem;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, velocities, time): Self::SystemData) {
        for (transform, velocity) in (&mut transforms, &velocities).join() {
            transform
                .set_translation_x(transform.translation().x + time.delta_seconds() * velocity.x);
            transform
                .set_translation_y(transform.translation().y + time.delta_seconds() * velocity.y);
        }
    }
}

/// Sets velocity for all entities with steering.
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Steering>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        Read<'s, DebugConfig>,
    );

    fn run(&mut self, (steerings, mut transforms, mut velocities, config): Self::SystemData) {
        for (transform, steering, velocity) in (&mut transforms, &steerings, &mut velocities).join()
        {
            let (centered_x, centered_y) = steering.to_centered_coords(steering.pos);
            if steering.grounded {
                // If grounded, correct y translation and zero out y velocity.
                transform.set_translation_y(centered_y);
                velocity.y = 0.0;
            } else {
                // If not, set y velocity.
                velocity.y = -config.player_speed;
            }

            // 1: Set velocity based on current position and desired position.
            // 2: If necessary, adjust position, snap to grid.

            let (desired_pos_x, _) = steering.to_centered_coords(steering.destination);
            let delta = desired_pos_x - transform.translation().x;
            let delta_signum = if delta.abs() < f32::EPSILON {
                0.0
            } else {
                delta.signum()
            };
            if (delta_signum * steering.direction).is_sign_positive() {
                velocity.x = delta_signum * config.player_speed;
            } else {
                velocity.x = 0.0;
                transform.set_translation_x(centered_x);
            }
        }
    }
}
