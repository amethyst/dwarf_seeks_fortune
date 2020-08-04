use amethyst::{
    assets::{Handle, Prefab},
    core::math::{Point2, Vector3},
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::sprite::SpriteRender,
    window::ScreenDimensions,
};

use crate::components::*;
use crate::levels::*;
use crate::resources::*;
use precompile::MyPrefabData;
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

/// Responsible for animating the cursor preview (IE the ghost of the block on the brush
/// that is displayed at the cursor position).
pub struct CursorPreviewSystem;

impl<'s> System<'s> for CursorPreviewSystem {
    type SystemData = (
        ReadStorage<'s, CursorPreviewTag>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, EditorData>,
    );

    fn run(&mut self, (tags, mut transforms, time, editor_data): Self::SystemData) {
        for (_, transform) in (&tags, &mut transforms).join() {
            let scale_factor = 1. - 0.1 * (time.absolute_time_seconds() * 3.14).sin().abs();
            transform.set_scale(Vector3::new(scale_factor, scale_factor, 1.0));
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
                        + min(editor_data.selector.start.x, editor_data.selector.end.x) as f32,
                    (height as f32 * 0.5)
                        + min(editor_data.selector.start.y, editor_data.selector.end.y) as f32,
                    (&DepthLayer::UiElements).z(),
                );
            }
        }
    }
}

/// Clears all dirty tiles, then adds all dirty tiles back in.
/// At the end of this system's execution, no tiles should be left dirty.
pub struct TilePaintSystem;

impl<'s> System<'s> for TilePaintSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Handle<Prefab<MyPrefabData>>>,
        WriteStorage<'s, PaintedTile>,
        Read<'s, TileDefinitions>,
        Read<'s, Assets>,
        Write<'s, EditorData>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            mut sprite_renders,
            mut anims,
            mut tiles,
            tile_defs,
            assets,
            mut data,
            entities,
        ): Self::SystemData,
    ) {
        for (_, entity) in (&tiles, &entities)
            .join()
            .filter(|(tile, _)| data.level.is_dirty(&tile.pos))
        {
            entities.delete(entity);
        }

        for (pos, tile_edit) in data
            .level
            .tile_map
            .iter_mut()
            .filter(|(pos, tile_edit)| tile_edit.dirty)
        {
            let tile_def = tile_defs.get(&tile_edit.tile_def_key);
            let still_asset = load_still_asset(tile_def, &assets);
            let anim_asset = load_anim_asset(tile_def, &assets);
            let transform = if let Some(asset) = &tile_def.asset {
                Some(load_transform(
                    &pos,
                    &tile_def.depth,
                    &tile_def.dimens,
                    asset,
                ))
            } else {
                None
            };
            let mut builder = entities.build_entity();
            if let Some(still_asset) = still_asset {
                builder = builder.with(still_asset, &mut sprite_renders);
            }
            if let Some(anim_asset) = anim_asset {
                builder = builder.with(anim_asset, &mut anims);
            }
            if let Some(transform) = transform {
                builder = builder.with(transform, &mut transforms);
            }
            builder
                .with(PaintedTile::new(pos.clone()), &mut tiles)
                .build();
            tile_edit.dirty = false;
        }
    }
}
