use amethyst::{
    core::math::{Point2, Vector3},
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

use crate::components::*;
use crate::levels::Map;
use crate::levels::*;
use crate::resources::*;
use std::cmp::min;

/// Responsible for moving the cursor across the screen.
pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Cursor>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Read<'s, EditorConfig>,
        Write<'s, EditorData>,
    );

    fn run(
        &mut self,
        (mut transforms, mut cursors, input, time, config, mut editor_data): Self::SystemData,
    ) {
        for (cursor, transform) in (&mut cursors, &mut transforms).join() {
            let input_x = input.axis_value("move_x").unwrap_or(0.0);
            let input_y = input.axis_value("move_y").unwrap_or(0.0);
            let new_direction = Direction::new(input_x, input_y);
            if cursor.last_direction.is_neutral() && !new_direction.is_neutral() {
                // Start movement now. Move once, then set cooldown to High.
                editor_data.selector.end.x += input_x as i32;
                editor_data.selector.end.y += input_y as i32;
                transform.set_translation_xyz(
                    editor_data.selector.end.x as f32 + 0.5,
                    editor_data.selector.end.y as f32 + 0.5,
                    2.0,
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
                        editor_data.selector.end.x as f32 + 0.5,
                        editor_data.selector.end.y as f32 + 0.5,
                        2.0,
                    );
                }
            }
            cursor.last_direction = new_direction;
        }
    }
}

/// Responsible for managing the selection.
pub struct SelectionSystem;

impl<'s> System<'s> for SelectionSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Cursor>,
        WriteStorage<'s, SelectionTag>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Write<'s, EditorData>,
    );

    fn run(
        &mut self,
        (mut transforms, mut cursors, mut selection_tags, input, time, mut editor_data): Self::SystemData,
    ) {
        let cursor_data = (&mut cursors)
            .join()
            .map(|cursor| cursor.last_direction)
            .next();
        if let Some(direction) = cursor_data {
            let shift = input.action_is_down("shift").unwrap_or(false);
            for (_, transform) in (&mut selection_tags, &mut transforms).join() {
                if !shift && !direction.is_neutral() {
                    editor_data.selector.start = editor_data.selector.end;
                }
                let width = (editor_data.selector.start.x - editor_data.selector.end.x).abs() + 1;
                let height = (editor_data.selector.start.y - editor_data.selector.end.y).abs() + 1;
                // TODO: set scale requires knowledge about dimensions of sprite.
                // Maybe solve with child entity.
                // Or accept hardcoded nature, because sprite unlikely to change?
                transform.set_scale(Vector3::new(width as f32 / 128., height as f32 / 128., 1.0));
                transform.set_translation_xyz(
                    (width as f32 * 0.5)
                        + min(editor_data.selector.start.x, editor_data.selector.end.x) as f32,
                    (height as f32 * 0.5)
                        + min(editor_data.selector.start.y, editor_data.selector.end.y) as f32,
                    1.0,
                );
            }
        }
    }
}

/// Deprecated.
pub struct TilePaintSystem;

impl<'s> System<'s> for TilePaintSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Pos>,
        WriteStorage<'s, Cursor>,
        WriteStorage<'s, SelectionTag>,
        WriteStorage<'s, PaintedTileTag>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Write<'s, Map>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            mut discrete_positions,
            mut cursors,
            mut selections,
            mut tiles,
            input,
            time,
            map,
            entities,
        ): Self::SystemData,
    ) {
        let enter = input.action_is_down("enter").unwrap_or(false);
        if !enter {
            return;
        }
    }
}
