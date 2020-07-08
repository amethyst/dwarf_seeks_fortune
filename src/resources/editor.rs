use crate::components::*;
use crate::levels::*;
use crate::resources::*;
use serde::{Deserialize, Serialize};
use std::cmp::min;

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct EditorConfig {
    pub cursor_move_high_cooldown: f32,
    pub cursor_move_low_cooldown: f32,
}

#[derive(Debug, Default)]
pub struct EditorData {
    pub brush: Brush,
    pub selector: Selector,
}

#[derive(Debug)]
pub struct Brush {
    pub tile: TileType,
}

//TODO: remove temporary default brush:
impl Default for Brush {
    fn default() -> Self {
        Brush {
            tile: TileType {
                dimens: Pos::new(1, 1),
                asset: AssetType::Still(SpriteType::Blocks, 0),
                entity_type: EntityType::DestructableTerrain,
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
