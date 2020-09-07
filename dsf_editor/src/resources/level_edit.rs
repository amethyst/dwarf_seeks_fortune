use std::collections::HashMap;

use crate::resources::TileEdit;
use dsf_core::components::Pos;
use dsf_core::levels::*;

/// The level editor uses this to store data related to the level it is editing.
/// An instance of LevelEdit can be transformed into a Level.
/// The main difference between this struct and the Level struct used by the game is
/// that this struct contains additional information that makes it easier to manipulate it.
#[derive(Debug, Clone)]
pub struct LevelEdit {
    pub pos: Pos,
    pub dimens: Pos,
    pub tile_map: HashMap<Pos, TileEdit>,
}

impl Default for LevelEdit {
    fn default() -> Self {
        LevelEdit {
            pos: Pos::new(-20, -10),
            dimens: Pos::new(40, 20),
            tile_map: HashMap::default(),
        }
    }
}

impl LevelEdit {
    pub fn put_tile(&mut self, pos: Pos, tile_edit: Option<TileEdit>) {
        if let Some(tile_edit) = tile_edit {
            self.tile_map.insert(pos, tile_edit);
        } else {
            self.tile_map.remove(&pos);
        }
    }
    pub fn is_dirty(&self, pos: &Pos) -> bool {
        self.tile_map
            .get(pos)
            .map(|tile_edit| tile_edit.dirty)
            .or(Some(true))
            .expect("Should never panic.")
    }
}

impl From<LevelEdit> for Level {
    fn from(mut item: LevelEdit) -> Self {
        let mut map = HashMap::new();
        item.tile_map.drain().for_each(|(key, val)| {
            map.insert(key, val.tile_def_key);
        });
        Level {
            pos: item.pos,
            dimens: item.dimens,
            tiles: map,
        }
    }
}

impl From<Level> for LevelEdit {
    fn from(mut item: Level) -> Self {
        let mut map = HashMap::new();
        item.tiles.drain().for_each(|(key, val)| {
            map.insert(key, TileEdit::new(val));
        });
        LevelEdit {
            pos: item.pos,
            dimens: item.dimens,
            tile_map: map,
        }
    }
}
