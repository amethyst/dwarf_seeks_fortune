use amethyst::{
    assets::{Handle, Prefab},
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::sprite::SpriteRender,
    Trans,
};

use crate::components::*;
use crate::resources::*;

use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{LazyUpdate, ReaderId, World};
use amethyst::input::{InputEvent, StringBindings, VirtualKeyCode};
use amethyst::prelude::WorldExt;
use amethyst::ui::UiEvent;
use dsf_core::components::{Background, Pos};
use dsf_core::levels::{
    load_anim_asset, load_still_asset, load_transform, TileDefinition, TileDefinitions,
};
use dsf_core::resources::{Assets, EventReaders};
use dsf_precompile::MyPrefabData;

pub struct PlaceTilesSystem;

impl<'s> System<'s> for PlaceTilesSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, EventReaders>,
        Read<'s, EventChannel<InputEvent<StringBindings>>>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (mut readers, event_channel, lazy): Self::SystemData) {
        let reader_id = readers
            .get_reader_id("place_tiles_system")
            .expect("ReaderId was not registered for system PlaceTilesSystem.");
        for event in event_channel.read(reader_id) {
            match event {
                InputEvent::KeyReleased {
                    key_code: VirtualKeyCode::Return,
                    scancode: _,
                } => {
                    lazy.exec_mut(|world| {
                        paint_tiles(world);
                    });
                }
                InputEvent::KeyReleased {
                    key_code: VirtualKeyCode::Delete,
                    scancode: _,
                } => {
                    lazy.exec_mut(|world| {
                        erase_tiles(world);
                    });
                }
                _ => (),
            }
        }
    }
}

pub fn paint_tiles(world: &mut World) {
    let (key, tile_def) = get_brush(world);
    set_tiles(world, key, tile_def);
}

pub fn erase_tiles(world: &mut World) {
    set_tiles(world, None, None);
}

fn set_tiles(world: &mut World, key: Option<String>, tile_def: Option<TileDefinition>) {
    let brush_dimens = tile_def
        .as_ref()
        .map(|def| def.dimens)
        .unwrap_or_else(|| Pos::new(1, 1));
    let mut editor_data = world.write_resource::<EditorData>();
    let lower_bounds = (*editor_data).selector.lower_bounds();
    let selection_dimens = (*editor_data).selector.dimens();
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

fn get_brush(world: &World) -> (Option<String>, Option<TileDefinition>) {
    let key = (*world.write_resource::<EditorData>())
        .brush
        .get_key()
        .clone();
    let def = key
        .as_ref()
        .map(|key| world.write_resource::<TileDefinitions>().get(key).clone());
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
