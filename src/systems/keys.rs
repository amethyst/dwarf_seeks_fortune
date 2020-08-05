use amethyst::core::ecs::Entities;
use amethyst::{
    core::math::Vector2,
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

use crate::components::*;
use crate::levels::*;
use crate::resources::*;

/// Key width and height, hardcoded for now.
const KEY_WIDTH: f32 = 2.;
const KEY_HEIGHT: f32 = 2.;

/// Checks if the player intersects any keys.
/// If so, the key will collected by the player and will be removed from the game.
#[derive(Default)]
pub struct WinSystem;

impl<'s> System<'s> for WinSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Steering>,
        ReadStorage<'s, Key>,
        ReadStorage<'s, KeyDisplay>,
        ReadStorage<'s, Transform>,
        Write<'s, WinCondition>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (player_tags, steerings, keys, key_displays, transforms, mut win, entities): Self::SystemData,
    ) {
        let player_collider = (&player_tags, &steerings, &transforms)
            .join()
            .map(|(_, steering, transform)| {
                (
                    Vector2::new(transform.translation().x, transform.translation().y),
                    Vector2::new(steering.dimens.x as f32, steering.dimens.y as f32),
                )
            })
            .next();
        if let Some((pos, dimens)) = player_collider {
            let collected_key = (&keys, &transforms, &entities)
                .join()
                .filter(|(_, transform, _)| {
                    let key_x = transform.translation().x;
                    let key_y = transform.translation().y;
                    pos.x - dimens.x / 2. < key_x + KEY_WIDTH / 3.
                        && pos.x + dimens.x / 2. > key_x - KEY_WIDTH / 3.
                        && pos.y - dimens.y / 2. < key_y + KEY_WIDTH / 3.
                        && pos.y + dimens.y / 2. > key_y - KEY_WIDTH / 3.
                })
                .map(|(key, _, entity)| (key, entity))
                .next();
            if let Some((key, key_entity)) = collected_key {
                win.set_key_collected(&key.pos);
                entities.delete(key_entity);
                for (key_display, display_entity) in (&key_displays, &entities).join() {
                    if key_display.pos == key.pos {
                        entities.delete(display_entity);
                    }
                }
            }
        }
    }
}
