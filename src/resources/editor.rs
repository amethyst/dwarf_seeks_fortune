use crate::components::*;
use crate::levels::*;
use crate::resources::*;
use amethyst::core::math::Point2;
use serde::{Deserialize, Serialize};

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
                dimens: Point2::new(1, 1),
                asset: AssetType::Still(SpriteType::Blocks, 0),
                entity_type: EntityType::DestructableTerrain,
            },
        }
    }
}

#[derive(Debug, Default)]
pub struct Selector {
    /// Inclusive bound.
    pub start: DiscretePos,
    /// Inclusive bound. The end point of the selection is always set to the current location of the cursor.
    pub end: DiscretePos,
}
