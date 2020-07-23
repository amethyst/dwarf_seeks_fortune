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

/// This system checks player input for movement and adjusts the player's steering accordingly.
pub struct PlayerSystem;

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
            let jump = input.action_is_down("jump").unwrap_or(false);

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
            } else if (steering.is_grounded() && !has_ground_beneath_feet)
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
            }

            // 1: Set current discrete position.
            // 2: Set steering based on user input.
            // TODO: Prioritise getting off the ladder when climbing.

            if steering.is_grounded() && input_x.abs() > f32::EPSILON {
                steering.direction = (input_x, 0.);
                let offset_from_discrete_pos = steering.pos.x as f32 - anchored_x;
                if offset_from_discrete_pos < f32::EPSILON && input_x > f32::EPSILON {
                    let climbing = check_climbing(input_y, anchored_y, steering, &tile_map);
                    if !climbing && !is_against_wall_right(&steering, anchored_y, &tile_map) {
                        steering.destination.x = steering.pos.x + 1;
                    }
                } else if offset_from_discrete_pos > -f32::EPSILON && input_x < f32::EPSILON {
                    let climbing = check_climbing(input_y, anchored_y, steering, &tile_map);
                    if !climbing && !is_against_wall_left(&steering, anchored_y, &tile_map) {
                        steering.destination.x = steering.pos.x - 1;
                    }
                } else if ((steering.destination.x - steering.pos.x) * input_x as i32).is_negative()
                {
                    // Player wants to go back where they came from.
                    steering.destination.x = steering.pos.x;
                }
            } else if (steering.is_grounded() || steering.is_climbing())
                && (anchored_x - steering.pos.x as f32).abs() < f32::EPSILON
            {
                let climbing = check_climbing(input_y, anchored_y, steering, &tile_map);
                if (anchored_y - steering.pos.y as f32).abs() < f32::EPSILON && !climbing {
                    steering.mode = SteeringMode::Grounded;
                }
            }

            // Push frame on history if player position changed.
            if old_pos != steering.pos || history.force_key_frame {
                history.push_frame(Frame::new(steering.pos.clone()));
            }
        }
    }
}

fn check_climbing(
    input_y: f32,
    anchored_y: f32,
    steering: &mut Steering,
    tile_map: &TileMap,
) -> bool {
    let offset_from_discrete_pos = steering.pos.y as f32 - anchored_y;
    if offset_from_discrete_pos < f32::EPSILON
        && input_y > f32::EPSILON
        && can_climb_up(steering, &tile_map)
    {
        steering.mode = SteeringMode::Climbing;
        steering.destination.y = steering.pos.y + 1;
        steering.direction = (0., 1.);
        true
    } else if offset_from_discrete_pos > -f32::EPSILON
        && input_y < -f32::EPSILON
        && can_climb_down(steering, &tile_map)
    {
        steering.mode = SteeringMode::Climbing;
        steering.destination.y = steering.pos.y - 1;
        steering.direction = (0., -1.);
        true
    } else if input_y.abs() > f32::EPSILON
        && ((steering.destination.y - steering.pos.y) * input_y as i32).is_negative()
    {
        // Player wants to go back where they came from.
        steering.destination.y = steering.pos.y;
        true
    } else {
        false
    }
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
