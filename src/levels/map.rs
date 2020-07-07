use crate::components::Pos;
use crate::resources::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Map {
    pub layers: Vec<TileLayer>,
}

impl Default for Map {
    fn default() -> Self {
        Map {
            layers: vec![TileLayer::default()],
        }
    }
}

impl Map {
    pub fn put_tile(&mut self, pos: Pos, tile: TileType) {
        self.layers
            .get_mut(0)
            .expect("There should be at least 1 layer in this Map.")
            .tiles
            .insert(pos, tile);
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct TileLayer {
    pub tiles: HashMap<Pos, TileType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tile {
    /// The position of the tile in tile coordinates.
    /// If this tile is more than 1x1 in width/height, then the position describes the
    /// bottom-left block in the tile.
    pub pos: Pos,
    pub tile_type: TileType,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct TileType {
    /// The width and height of the tile in tile coordinates.
    /// Most tiles will be 1 by 1. Some tiles, like mobs, may be 2 by 2.
    pub dimens: Pos,
    /// Which asset to draw on the screen? Will be stretched to fit the width and height
    /// set in 'dimens'.
    pub asset: AssetType,
    /// What kind of tile is this? Do we need to add any special components to the entity?
    pub entity_type: EntityType,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub enum EntityType {
    /// Collidable, static terrain. Doesn't move. Can be destroyed.
    DestructableTerrain,
    /// Collidable, static terrain. Doesn't move. Cannot be destroyed.
    IndestructableTerrain,
    /// Spawn a player here. Each level should contain exactly one of these.
    Player,
    /// Level key. The objective is to collect them all. Each level should contain at least one.
    Key,
    /// Spawns mobs from this location.
    MobSpawner,
    /// After collecting all keys, finish level by reaching this door.
    Door,
    /// Use these to climb up or down.
    Ladder,
}
