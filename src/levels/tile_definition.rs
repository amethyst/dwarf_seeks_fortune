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

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct TileDefinitions {
    pub map: HashMap<String, TileDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TileDefinition {
    /// How wide and high is the tile?
    pub dimens: Pos,
    /// This tile is unique in the level. Only one tile with this definition can appear in the level.
    /// Examples are the player and the exit door.
    pub unique: bool,
    /// This tile is mandatory for each level. A level cannot be played without at least one tile
    /// with this definition. Examples are the player and the exit door. Note that in combination
    /// with 'unique', a tile can be required to appear EXACTLY once in each level.
    pub mandatory: bool,
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
            dimens: Pos::new(1, 1),
            unique: false,
            mandatory: false,
            collision: None,
            asset: Some(AssetType::Still(SpriteType::NotFound, 0)),
            archetype: Archetype::NotFound,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
/// If there are any special rules that apply to this tile, the archetype signals this.
/// For example: a tile with the Archetype Player will be targeted by player input, etc.
///
/// TODO: Maybe change how this is defined. It's not complete and maybe too restricted.
///         For example, MobSpawner doesn't define which mobs it will spawn.
pub enum Archetype {
    /// ordinary block. Does nothing.
    Block,
    /// Spawn a player here.
    Player,
    /// Level key. The objective is to collect them all. Each level should contain at least one.
    Key,
    /// After collecting all keys, finish level by reaching this door.
    Door,
    /// Use these to climb up or down.
    Ladder,
    /// Spawns mobs from this location.
    MobSpawner,
    /// A fallback archetype used when an archetype lookup failed.
    NotFound,
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
    // TODO: Add special collision?
}

// Colliding:
// - Top collider: (blocks, ladders) Player can stand on them
// - Sides collider: (blocks) Player cannot move through them horizontally
// - Bottom collider: (blocks) Player cannot jump when under a 2-high overhang
// - Special:
//      - Ladder: Player can climb them.
//      - Key: Player collects them when moving through them.
//      - Tool: Player equips them when moving through if they don't already have tool.
//      - Door: Player wins level when moving through if they collected all keys.
//      - Trap wall: Wall appears if player moves through this.
//      - enemy: Player dies when moving through this collider.
//      - Trap floor: Will disappear when player moves across twice.
//      - Trap ladder: Will disappear when player moves through twice.
