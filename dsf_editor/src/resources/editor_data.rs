use crate::resources::{Brush, Selection};
use dsf_core::components::Pos;
use dsf_core::levels::LevelSave;
use dsf_core::resources::{Tile, TileDefinitions, TileMap};
use std::collections::{HashMap, HashSet};

/// Persists through play testing. Is only reset when the EditorState goes through on_create.
#[derive(Debug, Default)]
pub struct EditorData {
    /// Contains information on which tile is currently selected to be placed. Also contains the
    /// palette: all possible tiles that the editor could use.
    pub brush: Brush,
    /// The area that is currently selected.
    pub selection: Selection,
    /// If true, air will be included when copying a selection. In combination with the
    /// force_place flag, that means that copied air can clear out existing tiles.
    pub copy_air: bool,
    /// If true, existing tiles will be removed if they are in the way when placing tiles.
    /// If false, existing tiles will never be removed when placing tiles or pasting blueprints.
    ///     That means that it could happen that only part of the tiles are actually placed.
    pub force_place: bool,
}

#[derive(Debug, Default, Clone)]
pub struct LevelEdit {
    pub tile_map: TileMap,
    /// A list of tile positions that are marked are dirty and must be redrawn.
    /// Whenever you add, update or remove a tile in the editor, you must mark it as dirty.
    pub dirty: HashSet<Pos>,
}

impl From<LevelEdit> for LevelSave {
    fn from(mut item: LevelEdit) -> Self {
        let mut map = HashMap::new();
        item.tile_map.tiles.drain().for_each(|(pos, tile)| {
            if let Tile::TileDefKey(tile_def_key) = tile {
                map.insert(pos, tile_def_key);
            }
        });
        LevelSave {
            pos: item.tile_map.pos,
            dimens: item.tile_map.dimens,
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
        if self.tile_map.remove_tile(&pos) {
            self.dirty.insert(pos);
        }
    }

    fn add_tile(&mut self, pos: Pos, key: String, _force_place: bool) {
        self.tile_map.put_tile(pos, key);
        self.dirty.insert(pos);
    }
}
