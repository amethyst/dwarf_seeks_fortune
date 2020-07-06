use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

use crate::components::*;
use crate::resources::*;

const COOLDOWN_HIGH: f32 = 0.5;
const COOLDOWN_LOW: f32 = 0.05;

pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, DiscretePos>,
        WriteStorage<'s, Cursor>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut transforms, mut discrete_positions, mut cursors, input, time): Self::SystemData,
    ) {
        for (cursor, transform, discrete_pos) in
            (&mut cursors, &mut transforms, &mut discrete_positions).join()
        {
            let input_x = input.axis_value("move_x").unwrap_or(0.0);
            let input_y = input.axis_value("move_y").unwrap_or(0.0);
            let new_direction = Direction::new(input_x, input_y);
            if cursor.last_direction.is_neutral() && !new_direction.is_neutral() {
                // Start movement now. Move once, then set cooldown to High.
                discrete_pos.x += input_x as i32;
                discrete_pos.y += input_y as i32;
                transform.set_translation_xyz(
                    discrete_pos.x as f32 * 50. + 25.,
                    discrete_pos.y as f32 * 50. + 25.,
                    0.0,
                );
                cursor.cooldown = COOLDOWN_HIGH;
            } else if cursor.last_direction.is_opposite(&new_direction) {
                // Reset movement. Set cooldown to high.
                cursor.cooldown = COOLDOWN_HIGH;
            } else if !new_direction.is_neutral() {
                // continue movement. Tick down cooldown.
                // If cooldown is due, move once and reset cooldown to Low.
                cursor.cooldown -= time.delta_seconds();
                if cursor.cooldown.is_sign_negative() {
                    cursor.cooldown = COOLDOWN_LOW;
                    discrete_pos.x += input_x as i32;
                    discrete_pos.y += input_y as i32;
                    transform.set_translation_xyz(
                        discrete_pos.x as f32 * 50. + 25.,
                        discrete_pos.y as f32 * 50. + 25.,
                        0.0,
                    );
                }
            }
            cursor.last_direction = new_direction;
        }
    }
}
