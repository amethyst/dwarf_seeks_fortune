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
        WriteStorage<'s, Transform>,
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
            mut steerings,
            player_tags,
            input,
            config,
            tile_map,
            mut history,
        ): Self::SystemData,
    ) {
        for (_, transform, steering) in (&player_tags, &mut transforms, &mut steerings).join() {
            let old_pos = steering.pos.clone();
            steering.pos.x = calc_discrete_pos_x(transform);
            steering.pos.y = calc_discrete_pos_y(transform);

            let real_pos_y = transform.translation().y - 1.0;
            if real_pos_y <= steering.pos.y as f32 {
                // Check if I'm grounded.
                steering.grounded = is_grounded(&steering.pos, &tile_map);
            }

            // TODO: Climbing ladders....
            // let input_y = input.axis_value("move_y").unwrap_or(0.0);
            // if input_y.abs() > f32::EPSILON && is_on_ladder() {
            //     steering.destination.y += input_y;
            // }

            // 1: Set current discrete position.
            // 2: Set steering based on user input.

            // TODO: history frames .......
            if old_pos != steering.pos || history.force_key_frame {
                history.push_frame(Frame::new(steering.pos.clone()));
            }

            let input_x = input.axis_value("move_x").unwrap_or(0.0);
            if steering.grounded && input_x.abs() > f32::EPSILON {
                steering.direction = input_x;
                let offset_from_discrete_pos =
                    steering.pos.x as f32 - (transform.translation().x - 1.);
                if offset_from_discrete_pos < f32::EPSILON && input_x > f32::EPSILON {
                    if !is_against_wall_right(&steering.pos, &transform, &tile_map) {
                        steering.destination.x = steering.pos.x + 1;
                    }
                } else if offset_from_discrete_pos > -f32::EPSILON && input_x < f32::EPSILON {
                    if !is_against_wall_left(&steering.pos, &transform, &tile_map) {
                        steering.destination.x = steering.pos.x - 1;
                    }
                } else if ((steering.destination.x - steering.pos.x) * input_x as i32).is_negative()
                {
                    steering.destination.x = steering.pos.x;
                }
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
