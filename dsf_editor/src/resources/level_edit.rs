use dsf_core::components::Pos;
use dsf_core::levels::LevelSave;
use dsf_core::resources::{Tile, TileDefinition, TileDefinitions, TileMap, WorldBounds};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone)]
pub struct LevelEdit {
    pub tile_map: TileMap,
    /// A list of tile positions that are marked are dirty and must be redrawn.
    /// Whenever you add, update or remove a tile in the editor, you must mark it as dirty.
    pub dirty: HashSet<Pos>,
}

/// Implements the standard converter from LevelEdit to LevelSave. In other words: convert a level
/// from a format that the editor uses, to the format that levels are stored in on disk.
///
/// Note that only TileDefKey entries are saved, Dummy entries are discarded, because they can be
/// derived from the primary entries and only exist for faster lookup.
impl From<LevelEdit> for LevelSave {
    fn from(mut item: LevelEdit) -> Self {
        let mut map = HashMap::new();
        item.tile_map.tiles.drain().for_each(|(pos, tile)| {
            if let Tile::TileDefKey(tile_def_key) = tile {
                map.insert(pos, tile_def_key);
            }
        });
        LevelSave {
            world_bounds: item.tile_map.world_bounds,
            tiles: map,
        }
    }
}

impl LevelEdit {
    pub(crate) fn new(level_save: LevelSave, tile_defs: TileDefinitions) -> Self {
        let initial_dirty = level_save.tiles.keys().copied().collect::<HashSet<Pos>>();
        LevelEdit {
            tile_map: TileMap::for_editing(level_save, tile_defs),
            dirty: initial_dirty,
        }
    }

    /// Empty the set of dirty positions and collect them into a new collection.
    ///
    /// The process of re-collecting them before draining them (or otherwise iterating over them)
    /// again allows you to iterate over them without having an outstanding mutable borrow on the
    /// LevelEdit. This allows you to access the tile map while iterating.
    pub(crate) fn drain_dirty(&mut self) -> Vec<Pos> {
        self.dirty.drain().collect::<Vec<Pos>>()
    }

    pub(crate) fn put_tile(&mut self, force_place: bool, pos: Pos, tile: Option<Tile>) {
        match tile {
            Some(Tile::AirBlock) if force_place => self.remove_tile(pos),
            Some(Tile::TileDefKey(key)) => self.add_tile(pos, key, force_place),
            None => self.remove_tile(pos),
            _ => (),
        }
    }

    fn remove_tile(&mut self, pos: Pos) {
        if let Some(removed_pos) = self.tile_map.remove_tile(&pos) {
            self.dirty.insert(removed_pos);
        }
    }

    fn add_tile(&mut self, pos: Pos, key: String, force_place: bool) {
        let dimensions = self.get_tile_def(&key).dimens;
        if force_place {
            (0..dimensions.x).for_each(|x| {
                (0..dimensions.y).for_each(|y| {
                    self.remove_tile(pos.append_xy(x, y));
                })
            })
        }
        // TODO: check if we are allowed to place there. World bounds and existing tiles.
        self.tile_map.put_tile(pos, key, &dimensions);

        self.dirty.insert(pos);
    }

    pub fn get_tile_def(&self, key: &str) -> &TileDefinition {
        self.tile_map.tile_defs.get(key)
    }

    pub fn bounds(&self) -> &WorldBounds {
        &self.tile_map.world_bounds
    }
}
