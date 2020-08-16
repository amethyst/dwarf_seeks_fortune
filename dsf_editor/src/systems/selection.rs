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

use dsf_core::levels::DepthLayer;

use std::cmp::min;

/// Responsible for managing the selection.
pub struct SelectionSystem;

impl<'s> System<'s> for SelectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Cursor>,
        WriteStorage<'s, SelectionTag>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, EditorData>,
    );

    fn run(
        &mut self,
        (mut transforms, mut cursors, mut selection_tags, input, mut editor_data): Self::SystemData,
    ) {
        let cursor_data = (&mut cursors)
            .join()
            .map(|cursor| cursor.last_direction)
            .next();
        if let Some(direction) = cursor_data {
            let shift = input.action_is_down("shift").unwrap_or(false);
            for (_, transform) in (&mut selection_tags, &mut transforms).join() {
                if !shift && !direction.is_neutral() {
                    editor_data.selection.start = editor_data.selection.end;
                }
                let width = (editor_data.selection.start.x - editor_data.selection.end.x).abs() + 1;
                let height =
                    (editor_data.selection.start.y - editor_data.selection.end.y).abs() + 1;
                // TODO: set scale requires knowledge about dimensions of sprite.
                // Maybe solve with child entity.
                // Or accept hardcoded nature, because sprite unlikely to change?
                if width == 1 && height == 1 {
                    transform.set_scale(Vector3::new(0., 0., 1.0));
                } else {
                    transform.set_scale(Vector3::new(
                        width as f32 / 128.,
                        height as f32 / 128.,
                        1.0,
                    ));
                }

                transform.set_translation_xyz(
                    (width as f32 * 0.5)
                        + min(editor_data.selection.start.x, editor_data.selection.end.x) as f32,
                    (height as f32 * 0.5)
                        + min(editor_data.selection.start.y, editor_data.selection.end.y) as f32,
                    (&DepthLayer::UiElements).z(),
                );
            }
        }
    }
}
