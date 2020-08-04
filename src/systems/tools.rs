use amethyst::core::ecs::{Entities, LazyUpdate};
use amethyst::{
    core::math::{Vector2, Vector3},
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

use crate::components::*;
use crate::levels::*;
use crate::resources::*;
use amethyst::core::Parent;
use amethyst::prelude::{Builder, WorldExt};
use amethyst::renderer::SpriteRender;

/// Tool width and height, hardcoded for now.
/// TODO: Don't hardcode.
const TOOL_WIDTH: f32 = 2.;
const TOOL_HEIGHT: f32 = 2.;

/// Checks if the player intersects any tools.
/// If so, the tool will equipped by the player and will be removed from the game.
#[derive(Default)]
pub struct PickupSystem;

impl<'s> System<'s> for PickupSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        ReadStorage<'s, Steering>,
        ReadStorage<'s, Tool>,
        ReadStorage<'s, Transform>,
        Read<'s, LazyUpdate>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut players, steerings, tools, transforms, lazy, entities): Self::SystemData,
    ) {
        let mut player = (&mut players, &entities, &steerings, &transforms)
            .join()
            .map(|(player, entity, steering, transform)| {
                (
                    player,
                    entity,
                    Vector2::new(transform.translation().x, transform.translation().y),
                    Vector2::new(steering.dimens.x as f32, steering.dimens.y as f32),
                )
            })
            .next();
        if let Some((mut player, player_entity, pos, dimens)) = player {
            if player.equipped.is_some() {
                return;
            }
            let tool_opt = (&tools, &transforms, &entities)
                .join()
                .filter(|(_, transform, _)| {
                    let key_x = transform.translation().x;
                    let key_y = transform.translation().y;
                    pos.x - dimens.x / 2. < key_x + TOOL_WIDTH / 3.
                        && pos.x + dimens.x / 2. > key_x - TOOL_WIDTH / 3.
                        && pos.y - dimens.y / 2. < key_y + TOOL_WIDTH / 3.
                        && pos.y + dimens.y / 2. > key_y - TOOL_WIDTH / 3.
                })
                .next();
            if let Some((tool, _, tool_entity)) = tool_opt {
                player.equipped = Some(tool.tool_type);
                let (sprite, sprite_nr) = (tool.sprite, tool.sprite_nr);
                lazy.exec_mut(move |world| {
                    world
                        .delete_entity(tool_entity)
                        .expect("Tried to delete tool, but failed.");
                    let render = load_asset_from_world(&sprite, sprite_nr, world);
                    world
                        .create_entity()
                        .with(Transform::default())
                        .with(Parent {
                            entity: player_entity,
                        })
                        .with(render)
                        .build();
                });
            }
        }
    }
}

#[derive(Default)]
pub struct UseToolSystem;

impl<'s> System<'s> for UseToolSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        ReadStorage<'s, Steering>,
        ReadStorage<'s, Tool>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, LazyUpdate>,
        Write<'s, TileMap>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut players, steerings, tools, transforms, input, lazy, mut tile_map, entities): Self::SystemData,
    ) {
        //TODO: Maybe use SteeringIntent?
        let wants_to_use_tool = input.action_is_down("jump").unwrap_or(false);
        for (player, steering, transform) in (&mut players, &steerings, &transforms).join() {
            let (anchored_x, anchored_y) = steering.to_anchor_coords(transform);
            if wants_to_use_tool && steering.is_grounded() && player.equipped.is_some() {
                let blocks = tiles_to_side(2, steering);
                if blocks.iter().any(|pos| {
                    tile_map
                        .get_tile(pos)
                        .map(|block| block.is_breakable())
                        .unwrap_or(false)
                }) && blocks.iter().all(|pos| {
                    tile_map
                        .get_tile(pos)
                        .map(|block| block.is_breakable())
                        .unwrap_or(true)
                }) {
                    player.equipped = None;
                    blocks.iter().for_each(|pos| {
                        tile_map.remove_tile(pos);
                    })
                }
            }
        }
    }
}

fn tiles_to_side(depth: u8, steering: &Steering) -> Vec<Pos> {
    let x_offset = if steering.facing.x.is_positive() {
        steering.dimens.x
    } else {
        -1
    };
    (0..depth)
        .flat_map(|x| {
            (0..steering.dimens.y).map(move |y| (x as i32 * steering.facing.x.signum_i(), y))
        })
        .map(|(x, y)| Pos::new(steering.pos.x + x_offset + x, steering.pos.y + y))
        .collect()
}
