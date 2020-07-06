use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage, Entities},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

use crate::components::*;
use crate::resources::*;
use std::cmp::min;
use crate::levels::Map;

pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, DiscretePos>,
        WriteStorage<'s, Cursor>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Read<'s, EditorConfig>,
    );

    fn run(
        &mut self,
        (mut transforms, mut discrete_positions, mut cursors, input, time, config): Self::SystemData,
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
                    discrete_pos.x += input_x as i32;
                    discrete_pos.y += input_y as i32;
                    transform.set_translation_xyz(
                        discrete_pos.x as f32 * 50. + 25.,
                        discrete_pos.y as f32 * 50. + 25.,
                        2.0,
                    );
                }
            }
            cursor.last_direction = new_direction;
        }
    }
}

pub struct SelectionSystem;

impl<'s> System<'s> for SelectionSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, DiscretePos>,
        WriteStorage<'s, Cursor>,
        WriteStorage<'s, Selection>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut transforms, mut discrete_positions, mut cursors, mut selections, input, time): Self::SystemData,
    ) {
        let cursor_data = (&mut cursors, &mut discrete_positions)
            .join()
            .map(|(cursor, pos)| (cursor.last_direction, (pos.x, pos.y)))
            .next();
        if let Some((direction, (x, y))) = cursor_data {
            let shift = input.action_is_down("shift").unwrap_or(false);
            for (selection, transform) in (&mut selections, &mut transforms).join() {
                if !shift && !direction.is_neutral() {
                    selection.start = DiscretePos::new(x, y);
                }
                let width = (selection.start.x - x).abs() + 1;
                let height = (selection.start.y - y).abs() + 1;
                // TODO: set scale requires knowledge about dimensions of sprite.
                // Maybe solve with child entity.
                // Or accept hardcoded nature, because sprite unlikely to change?
                transform.set_scale(Vector3::new(
                    (width as f32 * 50.) / 128.,
                    (height as f32 * 50.) / 128.,
                    1.0,
                ));
                transform.set_translation_xyz(
                    (width as f32 * 25.) + (min(selection.start.x, x) as f32 * 50.),
                    (height as f32 * 25.) + (min(selection.start.y, y) as f32 * 50.),
                    1.0,
                );
            }
        }
    }
}

pub struct TilePaintSystem;

impl<'s> System<'s> for TilePaintSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, DiscretePos>,
        WriteStorage<'s, Cursor>,
        WriteStorage<'s, Selection>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Write<'s, Map>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut transforms, mut discrete_positions, mut cursors, mut selections, input, time, map, entities): Self::SystemData,
    ) {
        let enter = input.action_is_down("enter").unwrap_or(false);
        if !enter {
            return;
        }
    }
}
