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
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Steering>,
        WriteStorage<'s, Velocity>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, DebugConfig>,
        Read<'s, TileMap>,
        Write<'s, History>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            mut steerings,
            mut velocities,
            input,
            config,
            tile_map,
            mut history,
        ): Self::SystemData,
    ) {
        for (transform, steering, velocity) in
            (&mut transforms, &mut steerings, &mut velocities).join()
        {
            if steering.grounded {
                // If so, correct y translation and zero out y velocity.
                transform.set_translation_y(steering.pos.y as f32 + 1.0);
                velocity.y = 0.0;
            } else {
                // If not, update discrete y pos and set y velocity.
                velocity.y = -config.player_speed;
            }

            // 3: Set velocity based on current position and desired position.
            // 4: If necessary, adjust position, snap to grid.

            let desired_pos = steering.destination.x as f32 + 1.0;
            let delta = desired_pos - transform.translation().x;
            let delta_signum = if delta.abs() < f32::EPSILON {
                0.0
            } else {
                delta.signum()
            };
            if (delta_signum * steering.direction).is_sign_positive() {
                velocity.x = delta_signum * config.player_speed;
            } else {
                velocity.x = 0.0;
                transform.set_translation_x((steering.pos.x + 1) as f32);
            }
        }
    }
}
