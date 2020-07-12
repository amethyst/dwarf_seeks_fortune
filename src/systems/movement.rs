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

/// TODO: Maybe make discrete position part of steering?
/// TODO: Where do we set the discrete pos? Currently split between two systems.
/// This system checks player input for movement and adjusts the player's steering accordingly.
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Pos>,
        WriteStorage<'s, Steering>,
        ReadStorage<'s, PlayerTag>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, DebugConfig>,
        Read<'s, TileMap>,
        Write<'s, History>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            mut discrete_positions,
            mut steerings,
            player_tags,
            input,
            config,
            tile_map,
            mut history,
        ): Self::SystemData,
    ) {
        for (_, transform, discrete_pos, steering) in (
            &player_tags,
            &mut transforms,
            &mut discrete_positions,
            &mut steerings,
        )
            .join()
        {
            let real_pos_y = transform.translation().y - 1.0;
            if real_pos_y <= discrete_pos.y as f32 {
                // Check if I'm grounded.
                steering.grounded = is_grounded(&discrete_pos, &tile_map);
            }

            // TODO: Climbing ladders....
            // let input_y = input.axis_value("move_y").unwrap_or(0.0);
            // if input_y.abs() > f32::EPSILON && is_on_ladder() {
            //     steering.destination.y += input_y;
            // }

            // 1: Set current discrete position.
            // 2: Set steering based on user input.

            let old_pos = discrete_pos.clone();
            discrete_pos.x = calc_discrete_pos_x(transform);
            if old_pos != *discrete_pos || history.force_key_frame {
                history.push_frame(Frame::new(discrete_pos.clone()));
            }

            let input_x = input.axis_value("move_x").unwrap_or(0.0);
            if steering.grounded && input_x.abs() > f32::EPSILON {
                steering.direction = input_x;
                let offset_from_discrete_pos =
                    discrete_pos.x as f32 - (transform.translation().x - 1.);
                if offset_from_discrete_pos < f32::EPSILON && input_x > f32::EPSILON {
                    if !is_against_wall_right(&discrete_pos, &transform, &tile_map) {
                        steering.destination.x = discrete_pos.x + 1;
                    }
                } else if offset_from_discrete_pos > -f32::EPSILON && input_x < f32::EPSILON {
                    if !is_against_wall_left(&discrete_pos, &transform, &tile_map) {
                        steering.destination.x = discrete_pos.x - 1;
                    }
                } else if ((steering.destination.x - discrete_pos.x) * input_x as i32).is_negative()
                {
                    steering.destination.x = discrete_pos.x;
                }
            }
        }
    }
}

/// Sets velocity for all entities with steering.
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Pos>,
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
            mut discrete_positions,
            mut steerings,
            mut velocities,
            input,
            config,
            tile_map,
            mut history,
        ): Self::SystemData,
    ) {
        for (transform, discrete_pos, steering, velocity) in (
            &mut transforms,
            &mut discrete_positions,
            &mut steerings,
            &mut velocities,
        )
            .join()
        {
            if steering.grounded {
                // If so, correct y translation and zero out y velocity.
                transform.set_translation_y(discrete_pos.y as f32 + 1.0);
                velocity.y = 0.0;
            } else {
                // If not, update discrete y pos and set y velocity.
                discrete_pos.y = calc_discrete_pos_y(transform);
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
                transform.set_translation_x((discrete_pos.x + 1) as f32);
            }
        }
    }
}

/// Assumes the player is two-wide.
fn is_grounded(pos: &Pos, tile_map: &TileMap) -> bool {
    let tile_a = tile_map.get_tile(&Pos::new(pos.x, pos.y - 1));
    let tile_b = tile_map.get_tile(&Pos::new(pos.x + 1, pos.y - 1));
    tile_a.map(|tile| tile.provides_platform()).unwrap_or(false)
        || tile_b.map(|tile| tile.provides_platform()).unwrap_or(false)
}

fn is_against_wall_left(pos: &Pos, transform: &Transform, tile_map: &TileMap) -> bool {
    is_against_wall(&pos, &transform, &tile_map, -1)
}

fn is_against_wall_right(pos: &Pos, transform: &Transform, tile_map: &TileMap) -> bool {
    is_against_wall(&pos, &transform, &tile_map, 2) //Correction for width plus 1
}

fn is_against_wall(pos: &Pos, transform: &Transform, tile_map: &TileMap, x_offset: i32) -> bool {
    let pos_y = transform.translation().y - 1.;
    let floored_y = pos_y.floor();
    let nr_blocks_to_check = if (floored_y - pos_y).abs() > f32::EPSILON {
        3
    } else {
        2
    };
    (0..nr_blocks_to_check).any(|i| {
        // println!("i = {:?}", i);
        let tile = tile_map.get_tile(&Pos::new(pos.x + x_offset, floored_y as i32 + i));
        tile.map(|tile| tile.collides_horizontally())
            .unwrap_or(false)
    })
}

fn is_on_ladder() -> bool {
    true
}

fn calc_discrete_pos_x(transform: &Transform) -> i32 {
    let anchor_pos_x = transform.translation().x - 1.;
    anchor_pos_x.round() as i32
}

fn calc_discrete_pos_y(transform: &Transform) -> i32 {
    let anchor_pos_y = transform.translation().y - 1.;
    anchor_pos_y.round() as i32
}
