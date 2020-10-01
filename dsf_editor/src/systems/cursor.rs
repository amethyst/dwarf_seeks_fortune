use amethyst::shrev::EventChannel;
use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};

use dsf_core::components::Direction2D;
use dsf_core::resources::{SignalEdge, SignalEdgeDetector};

use crate::components::*;
use crate::resources::*;
use crate::systems::RefreshPreviewsEvent;

/// Responsible for moving the cursor across the screen and managing its blinking animation.
pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, EventChannel<RefreshPreviewsEvent>>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Cursor>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Read<'s, EditorConfig>,
        Read<'s, LevelEdit>,
        Write<'s, EditorData>,
        Write<'s, SignalEdgeDetector>,
    );

    fn run(
        &mut self,
        (
            mut channel,
            mut transforms,
            mut cursors,
            input,
            time,
            config,
            level_edit,
            mut editor_data,
            mut sed,
        ): Self::SystemData,
    ) {
        for (cursor, transform) in (&mut cursors, &mut transforms).join() {
            let shift = input.action_is_down("shift").unwrap_or(false);
            let input_x = input.axis_value("move_x").unwrap_or(0.0);
            let input_y = input.axis_value("move_y").unwrap_or(0.0);
            let new_direction = Direction2D::new(input_x, input_y);
            let should_move = if cursor.last_direction.is_neutral() && !new_direction.is_neutral() {
                // Start movement now. Move once and set cooldown to High.
                cursor.movement_cooldown = config.cursor_move_high_cooldown;
                true
            } else if cursor.last_direction.is_opposite(&new_direction) {
                // Reset movement. Set cooldown to high.
                cursor.movement_cooldown = config.cursor_move_high_cooldown;
                false
            } else if !new_direction.is_neutral() {
                // continue movement. Tick down cooldown.
                // If cooldown is due, move once and reset cooldown to Low.
                cursor.movement_cooldown -= time.delta_seconds();
                if cursor.movement_cooldown.is_sign_negative() {
                    cursor.movement_cooldown = config.cursor_move_low_cooldown;
                    true
                } else {
                    false
                }
            } else {
                false
            };
            cursor.last_direction = new_direction;
            let old_cursor_pos = editor_data.selection.end;
            if let SignalEdge::Rising = sed.edge("x_to_start", &input) {
                editor_data.selection.end.x = level_edit.bounds().x();
            }
            if let SignalEdge::Rising = sed.edge("x_to_end", &input) {
                editor_data.selection.end.x = level_edit.bounds().upper_x() - 1;
            }
            if let SignalEdge::Rising = sed.edge("y_to_start", &input) {
                editor_data.selection.end.y = level_edit.bounds().y();
            }
            if let SignalEdge::Rising = sed.edge("y_to_end", &input) {
                editor_data.selection.end.y = level_edit.bounds().upper_y() - 1;
            }
            if should_move {
                editor_data.selection.end.x += input_x as i32;
                editor_data.selection.end.y += input_y as i32;
            }
            editor_data.selection.end = level_edit.bounds().clamp(&editor_data.selection.end);
            if old_cursor_pos != editor_data.selection.end {
                transform.set_translation_x(editor_data.selection.end.x as f32 + 0.5);
                transform.set_translation_y(editor_data.selection.end.y as f32 + 0.5);
                reset_blink(cursor, &config);
                channel.single_write(RefreshPreviewsEvent);
                if !shift {
                    editor_data.selection.start = editor_data.selection.end;
                }
            }
            perform_blinking_animation(cursor, transform, &time, &config);
        }
    }
}

/// Resets the blinking cooldown, which ensures that the cursor stays visible.
/// Use when the cursor moves, so it is never invisible while the user is actively using it.
fn reset_blink(cursor: &mut Cursor, config: &EditorConfig) {
    if cursor.is_visible {
        cursor.blink_cooldown = config.cursor_blink_on_time;
    } else {
        cursor.blink_cooldown = 0.0;
    }
}

/// Tick down the blinking cooldown and take care of the the blinking animation if the cooldown is
/// ready.
fn perform_blinking_animation(
    cursor: &mut Cursor,
    transform: &mut Transform,
    time: &Time,
    config: &EditorConfig,
) {
    if cursor.blink_cooldown.is_sign_negative() {
        cursor.is_visible ^= true;
        cursor.blink_cooldown = if cursor.is_visible {
            config.cursor_blink_on_time
        } else {
            config.cursor_blink_off_time
        };
        let scale_factor = if cursor.is_visible { 1.0 } else { 0.0 };
        transform.set_scale(Vector3::new(scale_factor, scale_factor, 1.0));
    }
    cursor.blink_cooldown -= time.delta_seconds();
}
