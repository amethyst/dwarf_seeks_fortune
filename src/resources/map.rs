use serde::{Deserialize, Serialize};
use crate::resources::*;
use amethyst::{
    core::math::Point2,
};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Map {
    tiles: Vec<Tile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tile {
    /// The position of the tile in tile coordinates.
    /// If this tile is more than 1x1 in width/height, then the position describes the
    /// bottom-left block in the tile.
    pub pos: Point2<i32>,
    /// The width and height of the tile in tile coordinates.
    /// Most tiles will be 1 by 1. Some tiles, like mobs, may be 2 by 2.
    pub dimens: Point2<i32>,
    /// Which asset to draw on the screen? Will be stretched to fit the width and height
    /// set in 'dimens'.
    pub asset: AssetType,
    /// What kind of tile is this? Do we need to add any special components to the entity?
    pub tile_type: TileType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TileType {
    /// Collidable, static terrain. Doesn't move.
    StaticTile,
    /// Spawn a player here.
    PlayerStart,
    Key,
    MobSpawner,
    Door,
    Ladder,
}