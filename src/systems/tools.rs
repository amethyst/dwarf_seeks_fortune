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
        WriteStorage<'s, Transform>,
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
                lazy.exec_mut(move |world| {
                    world.delete_entity(tool_entity).unwrap();
                    let render = load_asset_from_world(&SpriteType::Tools, 2, world);
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
