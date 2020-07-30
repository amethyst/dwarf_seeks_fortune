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

/// How many seconds can pass between starting your jump and starting to move sideways for it to
/// still register. If you start moving sideways later than that, it will not work and the
/// character will simply jump straight up into the air instead.
const JUMP_ALLOWANCE: f32 = 0.1;

/// This system checks player input for movement and adjusts the player's steering accordingly.
/// TODO: Move the values in this struct to a component?
#[derive(Default)]
pub struct PlayerSystem {
    /// Whether the jump key is currently down. Needed to figure out if the player wants to jump
    /// this frame. (Jump is only executed if this value changes from false to true.)
    pressing_jump: bool,
    /// How many seconds have passed since the character started jumping?
    ///
    /// This value is usually None. When the character starts jumping, it is assigned Some(0.0).
    /// The delta_seconds is added to this value every tick. Once it surpasses a threshold, it is
    /// set back to None.
    ///
    /// As long as the grace timer hasn't run out yet, the player can give their jump horizontal
    /// speed. This fixes the problem that if the player presses jump and move at the same time,
    /// jump is sometimes registered before move and the character only jumps up, not sideways.
    jump_grace_timer: Option<f32>,
}

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        ReadStorage<'s, PlayerTag>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Steering>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, TileMap>,
        Write<'s, History>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (player_tags, transforms, mut steerings, input, tile_map, mut history, time): Self::SystemData,
    ) {
        for (_, transform, steering) in (&player_tags, &transforms, &mut steerings).join() {
            println!("Steering: {:?}", steering); // TODO: Remove later.
            let input_x = input.axis_value("move_x").unwrap_or(0.0);
            let input_y = input.axis_value("move_y").unwrap_or(0.0);
            let jump_down = input.action_is_down("jump").unwrap_or(false);
            let jump = jump_down && !self.pressing_jump;
            self.pressing_jump = jump_down;
            self.jump_grace_timer = if jump {
                Some(0.)
            } else if let Some(time_passed) = self.jump_grace_timer {
                let time_passed = time_passed + time.delta_seconds();
                if time_passed < JUMP_ALLOWANCE {
                    Some(time_passed)
                } else {
                    None
                }
            } else {
                None
            };

            let old_pos = steering.pos.clone();
            let (anchored_x, anchored_y) = steering.to_anchor_coords(transform);
            steering.pos = Pos::new(anchored_x.round() as i32, anchored_y.round() as i32);

            let has_ground_beneath_feet = is_grounded(&steering, &tile_map);
            if steering.is_falling()
                && anchored_y <= steering.pos.y as f32
                && has_ground_beneath_feet
                && on_solid_ground(steering, &tile_map)
            {
                // If falling and you reached the floor, set to grounded.
                steering.mode = SteeringMode::Grounded;
                steering.destination = steering.pos;
            } else if (steering.is_grounded()
                && !has_ground_beneath_feet
                && aligned_with_grid(steering.destination.x as f32, anchored_x, input_x))
                || (steering.is_climbing() && jump)
            {
                steering.mode = SteeringMode::Falling {
                    x_movement: 0.,
                    starting_y_pos: transform.translation().y,
                    starting_time: time.absolute_time_seconds(),
                };
            } else if steering.is_grounded() && jump {
                steering.mode = SteeringMode::Jumping {
                    x_movement: input_x,
                    starting_y_pos: transform.translation().y,
                    starting_time: time.absolute_time_seconds(),
                };
            } else if steering.jump_has_peaked(time.absolute_time_seconds()) {
                steering.mode = steering.mode.jump_to_fall();
            } else if steering.is_grounded()
                && aligned_with_grid(steering.destination.x as f32, anchored_x, input_x)
                && ((input_y > f32::EPSILON && can_climb_up(steering, &tile_map))
                    || (input_y < -f32::EPSILON && can_climb_down(steering, &tile_map)))
            {
                steering.mode = SteeringMode::Climbing;
            } else if steering.is_climbing()
                && aligned_with_grid(steering.destination.y as f32, anchored_y, input_y)
                && ((input_x > f32::EPSILON
                    && !is_against_wall_right(&steering, steering.pos.y as f32, &tile_map))
                    || (input_x < -f32::EPSILON
                        && !is_against_wall_left(&steering, steering.pos.y as f32, &tile_map)))
            {
                steering.mode = SteeringMode::Grounded;
            }

            match steering.mode {
                SteeringMode::Grounded => {
                    if input_x.abs() > f32::EPSILON {
                        steering.direction = (input_x, 0.);
                        let offset_from_discrete_pos = steering.destination.x as f32 - anchored_x;
                        if offset_from_discrete_pos < f32::EPSILON && input_x > f32::EPSILON {
                            if !is_against_wall_right(&steering, steering.pos.y as f32, &tile_map) {
                                steering.destination.x = steering.pos.x + 1;
                            }
                        } else if offset_from_discrete_pos > -f32::EPSILON && input_x < f32::EPSILON
                        {
                            if !is_against_wall_left(&steering, steering.pos.y as f32, &tile_map) {
                                steering.destination.x = steering.pos.x - 1;
                            }
                        } else if ((steering.destination.x - steering.pos.x) * input_x as i32)
                            .is_negative()
                        {
                            // Player wants to go back where they came from.
                            steering.destination.x = steering.pos.x;
                        }
                    }
                }
                SteeringMode::Climbing => {
                    if input_y.abs() > f32::EPSILON {
                        steering.direction = (0., input_y);
                        let offset_from_discrete_pos = steering.destination.y as f32 - anchored_y;
                        if offset_from_discrete_pos < f32::EPSILON
                            && input_y > f32::EPSILON
                            && can_climb_up(steering, &tile_map)
                        {
                            steering.destination.y = steering.pos.y + 1;
                        } else if offset_from_discrete_pos > -f32::EPSILON
                            && input_y < -f32::EPSILON
                        {
                            if can_climb_down(steering, &tile_map) {
                                steering.destination.y = steering.pos.y - 1;
                            } else if above_air(steering, &tile_map) {
                                steering.mode = SteeringMode::Falling {
                                    x_movement: 0.,
                                    starting_y_pos: transform.translation().y,
                                    starting_time: time.absolute_time_seconds(),
                                };
                            }
                        } else if ((steering.destination.y - steering.pos.y) * input_y as i32)
                            .is_negative()
                        {
                            // Player wants to go back where they came from.
                            steering.destination.y = steering.pos.y;
                        }
                    }
                }
                SteeringMode::Falling { x_movement, .. } => {
                    if x_movement.abs() < f32::EPSILON {
                        // No horizontal movement.
                        steering.destination.x = steering.pos.x;
                    } else if x_movement > f32::EPSILON {
                        // Moving towards the right.
                        if aligned_with_grid(steering.destination.x as f32, anchored_x, x_movement)
                            && !is_against_wall_right(steering, anchored_y, &tile_map)
                        {
                            steering.destination.x += 1;
                        }
                    } else {
                        // Moving towards the left.
                        if aligned_with_grid(steering.destination.x as f32, anchored_x, x_movement)
                            && !is_against_wall_left(steering, anchored_y, &tile_map)
                        {
                            steering.destination.x += -1;
                        }
                    }
                }
                SteeringMode::Jumping {
                    x_movement,
                    starting_y_pos,
                    starting_time,
                } => {
                    if self.jump_grace_timer.is_some() && input_x.abs() > f32::EPSILON {
                        steering.mode = SteeringMode::Jumping {
                            x_movement: input_x,
                            starting_y_pos,
                            starting_time,
                        };
                    }
                    if x_movement.abs() < f32::EPSILON {
                        // No horizontal movement.
                        steering.destination.x = steering.pos.x;
                    } else if x_movement > f32::EPSILON {
                        // Moving towards the right.
                        if aligned_with_grid(steering.destination.x as f32, anchored_x, x_movement)
                            && !is_against_wall_right(steering, anchored_y, &tile_map)
                        {
                            steering.destination.x += 1;
                        }
                    } else {
                        // Moving towards the left.
                        if aligned_with_grid(steering.destination.x as f32, anchored_x, x_movement)
                            && !is_against_wall_left(steering, anchored_y, &tile_map)
                        {
                            steering.destination.x += -1;
                        }
                    }
                }
            };

            // Push frame on history if player position changed.
            if old_pos != steering.pos || history.force_key_frame {
                history.push_frame(Frame::new(steering.pos.clone()));
            }
        }
    }
}

/// Returns true iff the player is aligned with the grid.
/// This function can be used for both horizontal and vertical coordinates.
fn aligned_with_grid(destination_pos: f32, actual_pos: f32, input: f32) -> bool {
    let offset = actual_pos - destination_pos;
    // Actual pos equal or greater than destination. Moving towards the positive.
    (offset > -f32::EPSILON && input > f32::EPSILON)
        // Actual pos equal or smaller than destination. Moving towards the negative.
        || (offset < f32::EPSILON && input < -f32::EPSILON)
        // Actual pos basically equal to the destination.
        || (offset.abs() < f32::EPSILON)
}

/// You cannot jump onto the middle of a ladder, so use this function to check if you
/// should set steering to Grounded.
/// Returns true iff entity is on solid ground; meaning the very top of a ladder or a proper,
/// solid block that is not climbable.
///
/// This definition excludes the middle of a ladder. While the middle of a ladder can be walked on,
/// it cannot be landed on from a jump or fall.
fn on_solid_ground(steering: &Steering, tile_map: &TileMap) -> bool {
    (0..steering.dimens.x).any(|i| {
        let tile = tile_map.get_tile(&Pos::new(steering.pos.x + i, steering.pos.y - 1));
        let tile_above = tile_map.get_tile(&Pos::new(steering.pos.x + i, steering.pos.y));
        tile.map(|tile| {
            tile.provides_platform()
                && (!tile.climbable
                    || !tile_above
                        .map(|tile_above| tile_above.climbable)
                        .unwrap_or(false))
        })
        .unwrap_or(false)
    })
}

fn is_grounded(steering: &Steering, tile_map: &TileMap) -> bool {
    (0..steering.dimens.x).any(|i| {
        let tile = tile_map.get_tile(&Pos::new(steering.pos.x + i, steering.pos.y - 1));
        tile.map(|tile| tile.provides_platform()).unwrap_or(false)
    })
}

fn is_against_wall_left(steering: &Steering, anchored_y: f32, tile_map: &TileMap) -> bool {
    is_against_wall(&steering, anchored_y, &tile_map, -1)
}

fn is_against_wall_right(steering: &Steering, anchored_y: f32, tile_map: &TileMap) -> bool {
    is_against_wall(&steering, anchored_y, &tile_map, steering.dimens.x)
}

fn is_against_wall(
    steering: &Steering,
    anchored_y: f32,
    tile_map: &TileMap,
    x_offset: i32,
) -> bool {
    let floored_y = anchored_y.floor();
    let nr_blocks_to_check = if (floored_y - anchored_y).abs() > f32::EPSILON {
        steering.dimens.y + 1
    } else {
        steering.dimens.y
    };
    (0..nr_blocks_to_check).any(|i| {
        let tile = tile_map.get_tile(&Pos::new(steering.pos.x + x_offset, floored_y as i32 + i));
        tile.map(|tile| tile.collides_horizontally())
            .unwrap_or(false)
    })
}

fn can_climb_up(steering: &Steering, tile_map: &TileMap) -> bool {
    can_climb(steering, tile_map, (0, 1))
}

fn can_climb_down(steering: &Steering, tile_map: &TileMap) -> bool {
    can_climb(steering, tile_map, (-1, 0))
}

fn can_climb(steering: &Steering, tile_map: &TileMap, y_range: (i32, i32)) -> bool {
    (0..steering.dimens.x).all(|x_offset| {
        (y_range.0..y_range.1).all(|y_offset| {
            let tile = tile_map.get_tile(&Pos::new(
                steering.pos.x + x_offset,
                steering.pos.y + y_offset,
            ));
            tile.map(|tile| tile.climbable).unwrap_or(false)
        })
    })
}

fn above_air(steering: &Steering, tile_map: &TileMap) -> bool {
    (0..steering.dimens.x).all(|x_offset| {
        let tile = tile_map.get_tile(&Pos::new(steering.pos.x + x_offset, steering.pos.y - 1));
        tile.map(|tile| !tile.provides_platform()).unwrap_or(true)
    })
}
