use crate::components::*;
use crate::levels::*;
use crate::resources::*;
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::collections::HashMap;

/// The level editor uses this to store data related to the level it is editing.
/// An instance of LevelEdit can be transformed into a Level.
/// The main difference between this struct and the Level struct used by the game is
/// that this struct contains additional information that makes it easier to manipulate it.
#[derive(Debug, Clone, Default)]
pub struct LevelEdit {
    pub tile_map: HashMap<Pos, TileEdit>,
}

impl LevelEdit {
    pub fn put_tile(&mut self, pos: Pos, tile_edit: TileEdit) {
        self.tile_map.insert(pos, tile_edit);
    }
}

impl From<LevelEdit> for Level {
    fn from(mut item: LevelEdit) -> Self {
        let mut map = HashMap::new();
        item.tile_map.drain().for_each(|(key, val)| {
            map.insert(key, val.tile_def_key);
        });
        Level { tile_defs: map }
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
    pub selector: Selector,
}

#[derive(Debug)]
pub struct Brush {
    pub tile_def_key: String,
    pub tile_def: TileDefinition,
}

//TODO: remove temporary default brush:
impl Default for Brush {
    fn default() -> Self {
        Brush {
            tile_def_key: String::from("Block1"),
            tile_def: TileDefinition {
                dimens: Pos::new(1, 1),
                unique: false,
                mandatory: false,
                collision: None,
                asset: Some(AssetType::Still(SpriteType::Blocks, 0)),
                archetype: Archetype::Block,
            },
        }
    }
}

/// TODO: Rename to Selection?
#[derive(Debug, Default)]
pub struct Selector {
    /// Inclusive bound.
    pub start: Pos,
    /// Inclusive bound. The end point of the selection is always set to the current location of the cursor.
    pub end: Pos,
}

impl Selector {
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
