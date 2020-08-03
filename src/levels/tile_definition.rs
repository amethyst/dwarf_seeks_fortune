use crate::components::Pos;
use crate::resources::{AssetType, SpriteType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Describes a complete level.
/// Contains a map of positions, mapped to tile definitions.
/// This struct can be loaded from a level file and used to start a game.
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Level {
    /// Mapping of (x,y) position in the world to a TileDefinition key.
    /// These keys can be used to look up the corresponding TileDefinition.
    pub tile_defs: HashMap<Pos, String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct TileDefinitions {
    /// Fallback value returned if a requested value cannot be found.
    fallback: TileDefinition,
    pub map: HashMap<String, TileDefinition>,
}

impl Default for TileDefinitions {
    fn default() -> Self {
        TileDefinitions {
            fallback: TileDefinition::fallback(),
            map: HashMap::default(),
        }
    }
}

impl<'a> TileDefinitions {
    pub fn get(&'a self, key: &str) -> &'a TileDefinition {
        self.map
            .get(key)
            .or_else(|| {
                error!("Failed to find tile definition {:?}, using fallback.", key);
                Some(&self.fallback)
            })
            .expect("Should never panic, because we use a fallback!")
    }
}

///TODO:
/// Name of struct. Tile?Block? What do we call the dimens unit? Tile? Meter? Grid space?
/// asset not necessarily the same as the asset in editor. (Mob spawn)
/// this tile map resource that is created, doesn't need to contain ALL tiles.
///
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct TileDefinition {
    pub depth: DepthLayer,
    /// How wide and high is the tile?
    pub dimens: Pos,
    /// This tile is unique in the level. Only one tile with this definition can appear in the level.
    /// Examples are the player and the exit door.
    pub unique: bool,
    /// This tile is mandatory for each level. A level cannot be played without at least one tile
    /// with this definition. Examples are the player and the exit door. Note that in combination
    /// with 'unique', a tile can be required to appear EXACTLY once in each level.
    pub mandatory: bool,
    /// Whether you can climb up and down on this block. For ladders, this is true.
    pub climbable: bool,
    /// Collision data for the tile. Is optional, because not all tiles collide.
    pub collision: Option<CollisionDefinition>,
    /// The graphical asset to use for this tile. Is optional, because not all tiles have an asset.
    pub asset: Option<AssetType>,
    pub archetype: Archetype,
}

impl TileDefinition {
    /// Use the fallback if the real TileDefinition could not be found.
    /// This avoids the game having to panic if a level file is slightly corrupted or out of date.
    pub fn fallback() -> Self {
        TileDefinition {
            depth: DepthLayer::Blocks,
            dimens: Pos::new(1, 1),
            unique: false,
            mandatory: false,
            climbable: false,
            collision: None,
            asset: Some(AssetType::Still(SpriteType::NotFound, 0)),
            archetype: Archetype::NotFound,
        }
    }

    /// True if and only if the tile collides at the top.
    /// In other words, if you can stand on top of this tile.
    pub fn provides_platform(&self) -> bool {
        if let Some(collision) = &self.collision {
            collision.collides_top
        } else {
            false
        }
    }

    /// True if and only if the tile collides on the right and left sides.
    pub fn collides_horizontally(&self) -> bool {
        if let Some(collision) = &self.collision {
            collision.collides_side
        } else {
            false
        }
    }

    /// True if and only if the tile collides on the bottom.
    pub fn collides_bottom(&self) -> bool {
        if let Some(collision) = &self.collision {
            collision.collides_bottom
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DepthLayer {
    Background,
    // Leave room to expand background into multiple parallax layers here
    Blocks,
    FloatingBlocks,
    Enemies,
    Player,
    Particles,
    /// Any UI elements that exist in world space. If we'd want a health bar above an enemy's head,
    /// this is the z-layer we'd use.
    /// Currently, it is used for some debugging elements, such as the player frames.
    /// In the editor, it is used for the cursor and selection.
    UiElements,
    Camera,
}

impl Default for DepthLayer {
    fn default() -> Self {
        DepthLayer::Blocks
    }
}

impl DepthLayer {
    pub fn z(&self) -> f32 {
        match self {
            DepthLayer::Background => 0.,
            DepthLayer::Blocks => 100.,
            DepthLayer::FloatingBlocks => 110.,
            DepthLayer::Enemies => 120.,
            DepthLayer::Player => 130.,
            DepthLayer::Particles => 140.,
            DepthLayer::UiElements => 200.,
            DepthLayer::Camera => 300.,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
/// If there are any special rules that apply to this tile, the archetype signals this.
/// For example: a tile with the Archetype Player will be targeted by player input, etc.
///
/// Use a different archetype to attach special components or sub-entities to an entity.
pub enum Archetype {
    /// ordinary block. Does nothing.
    Block,
    /// Spawn a player here.
    Player,
    /// Level key. The objective is to collect them all. Each level should contain at least one.
    Key,
    /// After collecting all keys, finish level by reaching this door.
    Door,
    /// Spawns mobs from this location.
    MobSpawner,
    /// A fallback archetype used when an archetype lookup failed.
    NotFound,
    Tool(ToolType),
}

impl Default for Archetype {
    fn default() -> Self {
        Archetype::Block
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum ToolType {
    PickAxe,
    Hammer,
}

impl Default for ToolType {
    fn default() -> Self {
        ToolType::PickAxe
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CollisionDefinition {
    /// Player can stand on these tiles. Examples include regular blocks and ladders.
    pub collides_top: bool,
    /// Player cannot move through these tiles horizontally. Examples include blocks.
    pub collides_side: bool,
    /// When standing underneath a two-high ledge of these tiles, the player cannot jump.
    pub collides_bottom: bool,
}
