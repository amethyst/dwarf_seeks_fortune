use amethyst::core::ecs::shrev::EventChannel;

use amethyst::input::{InputEvent, StringBindings, VirtualKeyCode};
use amethyst::prelude::WorldExt;

use amethyst::{
    assets::{Handle, Prefab},
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::sprite::SpriteRender,
};

use dsf_core::components::{Background, Pos};
use dsf_core::levels::{
    load_anim_asset, load_still_asset, load_transform, TileDefinition, TileDefinitions,
};
use dsf_core::resources::{Assets, EventReaders};
use dsf_precompile::MyPrefabData;

use crate::components::*;
use crate::resources::*;

pub struct PlaceTilesSystem;

/// TODO: Delay in channel is unacceptable here. Replace channel with direct input check.
impl<'s> System<'s> for PlaceTilesSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, EventReaders>,
        Read<'s, EventChannel<InputEvent<StringBindings>>>,
        Read<'s, TileDefinitions>,
        Write<'s, EditorData>,
    );

    fn run(&mut self, (mut readers, event_channel, tile_defs, mut editor_data): Self::SystemData) {
        let reader_id = readers
            .get_reader_id("place_tiles_system")
            .expect("ReaderId was not registered for system PlaceTilesSystem.");
        for event in event_channel.read(reader_id) {
            match event {
                InputEvent::KeyReleased {
                    key_code: VirtualKeyCode::Return,
                    scancode: _,
                } => {
                    let (key, tile_def) = get_brush(&editor_data, &tile_defs);
                    set_tiles(&mut editor_data, key, tile_def);
                }
                InputEvent::KeyReleased {
                    key_code: VirtualKeyCode::Delete,
                    scancode: _,
                } => {
                    set_tiles(&mut editor_data, None, None);
                }
                _ => (),
            }
        }
    }
}

fn set_tiles(editor_data: &mut EditorData, key: Option<String>, tile_def: Option<TileDefinition>) {
    let brush_dimens = tile_def
        .as_ref()
        .map(|def| def.dimens)
        .unwrap_or_else(|| Pos::new(1, 1));
    let lower_bounds = (*editor_data).selection.lower_bounds();
    let selection_dimens = (*editor_data).selection.dimens();
    for x in
        (lower_bounds.x..(lower_bounds.x + selection_dimens.x)).step_by(brush_dimens.x as usize)
    {
        for y in
            (lower_bounds.y..(lower_bounds.y + selection_dimens.y)).step_by(brush_dimens.y as usize)
        {
            (*editor_data)
                .level
                .put_tile(Pos::new(x, y), key.clone().map(TileEdit::new));
        }
    }
}

fn get_brush(
    editor_data: &EditorData,
    tile_defs: &TileDefinitions,
) -> (Option<String>, Option<TileDefinition>) {
    let key = editor_data.brush.get_key().clone();
    let def = key.as_ref().map(|key| tile_defs.get(key).clone());
    (key, def)
}

/// Clears all dirty tiles, then adds all dirty tiles back in.
/// At the end of this system's execution, no tiles should be left dirty.
pub struct TilePaintSystem;

impl<'s> System<'s> for TilePaintSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Background>,
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
            _backgrounds,
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
            entities
                .delete(entity)
                .expect("Failed to delete tile sprite.");
        }

        for (pos, tile_edit) in data
            .level
            .tile_map
            .iter_mut()
            .filter(|(_, tile_edit)| tile_edit.dirty)
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
            builder.with(PaintedTile::new(*pos), &mut tiles).build();
            tile_edit.dirty = false;
        }
    }
}
