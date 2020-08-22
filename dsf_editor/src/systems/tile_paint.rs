use amethyst::{
    assets::{Handle, Prefab},
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::sprite::SpriteRender,
};

use dsf_core::components::Background;
use dsf_core::levels::{load_anim_asset, load_still_asset, load_transform, TileDefinitions};
use dsf_core::resources::Assets;
use dsf_precompile::MyPrefabData;

use crate::components::*;
use crate::resources::*;

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
