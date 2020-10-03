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
        let (to_be_placed, to_be_deleted) = self.check_place_tile(force_place, pos, tile);
        println!("({:?}, {:?})", to_be_placed, to_be_deleted);
        to_be_deleted.iter().for_each(|delete_pos| {
            if let Some(removed_pos) = self.tile_map.remove_tile(&delete_pos) {
                self.dirty.insert(removed_pos);
            }
        });
        if let Some((pos, key, dimensions)) = to_be_placed {
            self.tile_map.put_tile(pos, key, &dimensions);
            self.dirty.insert(pos);
        }
    }

    pub(crate) fn check_place_tile(
        &self,
        force_place: bool,
        pos: Pos,
        tile: Option<Tile>,
    ) -> (Option<(Pos, String, Pos)>, HashSet<Pos>) {
        match tile {
            Some(Tile::AirBlock) if force_place => (
                None,
                self.tile_map.get_actual_pos(&pos).iter().copied().collect(),
            ),
            Some(Tile::TileDefKey(key)) => self.check_add_tile(pos, key, force_place),
            None => (
                None,
                self.tile_map.get_actual_pos(&pos).iter().copied().collect(),
            ),
            _ => (None, HashSet::default()),
        }
    }

    fn check_add_tile(
        &self,
        pos: Pos,
        key: String,
        force_place: bool,
    ) -> (Option<(Pos, String, Pos)>, HashSet<Pos>) {
        let dimensions = self.get_tile_def(&key).dimens;
        let obstructed = !self.bounds().encloses(&pos, &dimensions)
            || (!force_place
                && (0..dimensions.x).any(|x| {
                    (0..dimensions.y).any(|y| {
                        self.tile_map
                            .get_tile(&Pos::new(pos.x + x, pos.y + y))
                            .is_some()
                    })
                }));
        if obstructed {
            (None, HashSet::default())
        } else {
            let to_be_deleted = (0..dimensions.x)
                .flat_map(|x| (0..dimensions.y).map(move |y| (x, y)))
                .map(|(x, y)| {
                    self.tile_map
                        .get_actual_pos(&Pos::new(pos.x + x, pos.y + y))
                })
                .flatten()
                .collect();
            (Some((pos, key, dimensions)), to_be_deleted)
        }
    }

    pub(crate) fn get_tile_def(&self, key: &str) -> &TileDefinition {
        self.tile_map.tile_defs.get(key)
    }

    pub(crate) fn bounds(&self) -> &WorldBounds {
        &self.tile_map.world_bounds
    }
}
