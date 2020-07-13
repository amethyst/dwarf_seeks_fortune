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
    );

    fn run(
        &mut self,
        (player_tags, transforms, mut steerings, input, tile_map, mut history): Self::SystemData,
    ) {
        for (_, transform, steering) in (&player_tags, &transforms, &mut steerings).join() {
            let old_pos = steering.pos.clone();
            let (anchored_x, anchored_y) = steering.to_anchor_coords(transform);
            steering.pos = Pos::new(anchored_x.round() as i32, anchored_y.round() as i32);

            if anchored_y <= steering.pos.y as f32 {
                // Check if I'm grounded.
                steering.grounded = is_grounded(&steering, &tile_map);
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
                let offset_from_discrete_pos = steering.pos.x as f32 - anchored_x;
                if offset_from_discrete_pos < f32::EPSILON && input_x > f32::EPSILON {
                    if !is_against_wall_right(&steering, anchored_y, &tile_map) {
                        steering.destination.x = steering.pos.x + 1;
                    }
                } else if offset_from_discrete_pos > -f32::EPSILON && input_x < f32::EPSILON {
                    if !is_against_wall_left(&steering, anchored_y, &tile_map) {
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

fn is_on_ladder() -> bool {
    true
}
