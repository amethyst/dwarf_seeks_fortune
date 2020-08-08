use crate::components::*;

use crate::resources::*;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
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
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Steering>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        Read<'s, MovementConfig>,
        Read<'s, Time>,
    );

    fn run(&mut self, (steerings, mut transforms, mut velocities, config, time): Self::SystemData) {
        for (transform, steering, velocity) in (&mut transforms, &steerings, &mut velocities).join()
        {
            let (centered_x, centered_y) = steering.to_centered_coords(steering.pos);
            let (desired_pos_x, desired_pos_y) = steering.to_centered_coords(steering.destination);
            match steering.mode {
                SteeringMode::Grounded => {
                    // If grounded, correct y translation and zero out y velocity.
                    transform.set_translation_y(centered_y);
                    velocity.y = 0.0;
                }
                SteeringMode::Climbing => {
                    // If climbing, correct x translation and zero out x velocity.
                    transform.set_translation_x(centered_x);
                    velocity.x = 0.0;
                    // If climbing:
                    let delta = desired_pos_y - transform.translation().y;
                    if steering.direction.y.aligns_with(delta) {
                        velocity.y = steering.direction.y.signum() * config.player_speed;
                    } else {
                        velocity.y = 0.0;
                        transform.set_translation_y(centered_y);
                    }
                }
                SteeringMode::Falling { starting_y_pos, .. } => {
                    // Set y-position directly, based on movement function. We don't use velocity for this.
                    velocity.y = 0.0;
                    transform.set_translation_y(
                        starting_y_pos + steering.mode.calc_delta_y(time.absolute_time_seconds()),
                    );
                }
                SteeringMode::Jumping { starting_y_pos, .. } => {
                    // Set y-position directly, based on movement function. We don't use velocity for this.
                    velocity.y = 0.0;
                    transform.set_translation_y(
                        starting_y_pos + steering.mode.calc_delta_y(time.absolute_time_seconds()),
                    );
                }
            }

            // Set x-velocity based on current and desired position.
            // If necessary, adjust x-position, snap to grid.
            let delta = desired_pos_x - transform.translation().x;
            if steering.direction.x.aligns_with(delta) {
                velocity.x = steering.direction.x.signum() * config.player_speed;
            } else {
                velocity.x = 0.0;
                transform.set_translation_x(centered_x);
            }
        }
    }
}

/// Implements wrapping behaviour for levels.
///
/// IE: if character falls out the bottom, they appear at top. If character goes off to the left,
/// they wrap around to the right.
pub struct LevelWrappingSystem;

impl<'s> System<'s> for LevelWrappingSystem {
    type SystemData = (
        WriteStorage<'s, Steering>,
        WriteStorage<'s, Transform>,
        Read<'s, TileMap>,
    );

    fn run(&mut self, (mut steerings, mut transforms, tile_map): Self::SystemData) {
        for (transform, steering) in (&mut transforms, &mut steerings).join() {
            // TODO: also implement for left, right, top borders.
            let (anchored_x, anchored_y) = steering.to_anchor_coords(transform);
            if anchored_y < tile_map.pos.y as f32 {
                transform.set_translation_y(transform.translation().y + tile_map.dimens.y as f32);
                match steering.mode {
                    SteeringMode::Falling {
                        x_movement,
                        starting_y_pos,
                        starting_time,
                    } => {
                        steering.mode = SteeringMode::Falling {
                            x_movement,
                            starting_y_pos: starting_y_pos + tile_map.dimens.y as f32,
                            starting_time,
                        }
                    }
                    _ => (),
                };
            }
            if anchored_x < tile_map.pos.x as f32 {
                transform.set_translation_x(transform.translation().x + tile_map.dimens.x as f32);
            } else if anchored_x > (tile_map.pos.x + tile_map.dimens.x) as f32 {
                transform.set_translation_x(transform.translation().x - tile_map.dimens.x as f32);
            }
        }
    }
}
