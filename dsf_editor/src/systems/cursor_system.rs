use amethyst::{
    assets::{Handle, Prefab},
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::sprite::SpriteRender,
};

use crate::components::*;
use crate::resources::*;

use dsf_core::components::Direction2D;

/// Responsible for moving the cursor across the screen.
pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Cursor>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Read<'s, EditorConfig>,
        Write<'s, EditorData>,
    );

    // TODO: Some code duplication here.
    fn run(
        &mut self,
        (mut transforms, mut cursors, input, time, config, mut editor_data): Self::SystemData,
    ) {
        for (cursor, transform) in (&mut cursors, &mut transforms).join() {
            let input_x = input.axis_value("move_x").unwrap_or(0.0);
            let input_y = input.axis_value("move_y").unwrap_or(0.0);
            let new_direction = Direction2D::new(input_x, input_y);
            if cursor.last_direction.is_neutral() && !new_direction.is_neutral() {
                // Start movement now. Move once, then set cooldown to High.
                editor_data.selector.end.x += input_x as i32;
                editor_data.selector.end.y += input_y as i32;
                transform.set_translation_xyz(
                    editor_data.selector.end.x as f32,
                    editor_data.selector.end.y as f32,
                    0.0,
                );
                cursor.cooldown = config.cursor_move_high_cooldown;
            } else if cursor.last_direction.is_opposite(&new_direction) {
                // Reset movement. Set cooldown to high.
                cursor.cooldown = config.cursor_move_high_cooldown;
            } else if !new_direction.is_neutral() {
                // continue movement. Tick down cooldown.
                // If cooldown is due, move once and reset cooldown to Low.
                cursor.cooldown -= time.delta_seconds();
                if cursor.cooldown.is_sign_negative() {
                    cursor.cooldown = config.cursor_move_low_cooldown;
                    editor_data.selector.end.x += input_x as i32;
                    editor_data.selector.end.y += input_y as i32;
                    transform.set_translation_xyz(
                        editor_data.selector.end.x as f32,
                        editor_data.selector.end.y as f32,
                        0.0,
                    );
                }
            }
            cursor.last_direction = new_direction;
        }
    }
}
