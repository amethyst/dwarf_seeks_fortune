use std::cmp::min;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Default)]
pub struct TileEdit {
    pub tile_def_key: String,
    pub dirty: bool,
}

impl TileEdit {
    pub fn new(tile_def_key: String) -> Self {
        TileEdit {
            tile_def_key,
            dirty: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct EditorConfig {
    pub cursor_move_high_cooldown: f32,
    pub cursor_move_low_cooldown: f32,
}

#[derive(Debug, Default)]
pub struct EditorData {
    pub level: LevelEdit,
    pub brush: Brush,
    pub selection: Selection,
}

#[derive(Debug, Default)]
pub struct Brush {
    palette: Vec<Option<String>>,
    palette_index: usize,
}

impl Brush {
    pub fn set_palette(&mut self, defs: &TileDefinitions) {
        self.palette.clear();
        self.palette.push(None);
        defs.map.keys().for_each(|key| {
            self.palette.push(Some(key.clone()));
        });
        self.palette.sort();
    }
    pub fn select_previous(&mut self) -> Option<String> {
        self.select(-1)
    }
    pub fn select_next(&mut self) -> Option<String> {
        self.select(1)
    }

    fn select(&mut self, offset: i32) -> Option<String> {
        self.palette_index =
            (self.palette_index as i32 + offset).rem_euclid(self.palette.len() as i32) as usize;
        let new_key = self.get_key();
        info!("Selected brush: {:?}", new_key);
        new_key.clone()
    }

    pub fn get_key(&self) -> &Option<String> {
        self.palette
            .get(self.palette_index)
            .expect("Should not panic.")
    }
}

#[derive(Debug, Default)]
pub struct Selection {
    /// Inclusive bound.
    pub start: Pos,
    /// Inclusive bound. The end point of the selection is always set to the current location of the cursor.
    pub end: Pos,
}

impl Selection {
    pub fn lower_bounds(&self) -> Pos {
        Pos::new(min(self.start.x, self.end.x), min(self.start.y, self.end.y))
    }
    pub fn dimens(&self) -> Pos {
        Pos::new(
            (self.start.x - self.end.x).abs() + 1,
            (self.start.y - self.end.y).abs() + 1,
        )
    }
}
