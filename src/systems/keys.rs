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
        ReadStorage<'s, PlayerTag>,
        ReadStorage<'s, Steering>,
        ReadStorage<'s, KeyTag>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
    );

    fn run(&mut self, (player_tags, steerings, key_tags, transforms, entities): Self::SystemData) {
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
            for (_, transform, entity) in (&key_tags, &transforms, &entities).join() {
                let key_x = transform.translation().x;
                let key_y = transform.translation().y;
                if pos.x - dimens.x / 2. < key_x + KEY_WIDTH / 3.
                    && pos.x + dimens.x / 2. > key_x - KEY_WIDTH / 3.
                    && pos.y - dimens.y / 2. < key_y + KEY_WIDTH / 3.
                    && pos.y + dimens.y / 2. > key_y - KEY_WIDTH / 3.
                {
                    entities.delete(entity);
                }
            }
        }
    }
}
