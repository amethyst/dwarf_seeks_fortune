use crate::components::*;
use crate::levels::*;
use crate::resources::{TileDefinition, TileDefinitions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct TileMap {
    // TODO: refactor these into a WorldBounds struct?
    pub pos: Pos,
    pub dimens: Pos,
    pub tiles: HashMap<Pos, Tile>,
    pub tile_defs: TileDefinitions,
}

impl TileMap {
    /// Construct a TileMap for use during the PlayState.
    /// Keeps track of some relevant tiles only: climbable, collidable, destructable tiles.
    pub fn for_play(level: LevelSave, tile_defs: TileDefinitions) -> Self {
        TileMap::new(level, tile_defs, true)
    }

    pub fn for_editing(level: LevelSave, tile_defs: TileDefinitions) -> Self {
        TileMap::new(level, tile_defs, false)
    }

    fn new(level: LevelSave, tile_defs: TileDefinitions, apply_filter: bool) -> Self {
        let mut tiles = HashMap::new();
        level.tiles.iter()
            .map(|(pos, key)| {
                (pos, key, tile_defs.get(key))
            })
            .filter(|(_, _, tile_def)| {
                // Make sure we only add relevant stuff to the tile map.
                !apply_filter || tile_def.climbable || tile_def.collision.is_some() || tile_def.is_breakable()
            })
            .for_each(|(pos, key, tile_def)| {
                let dimens = tile_def.dimens;
                for x in 0..dimens.x {
                    for y in 0..dimens.y {
                        let tile = if x == 0 && y == 0 {
                            Tile::TileDefKey(key.clone())
                        } else {
                            Tile::Dummy(*pos)
                        };
                        if let Some(replaced_value) = tiles.insert(Pos::new(pos.x + x, pos.y + y), tile) {
                            error!("Error! At pos ({:?},{:?}), there are multiple tiles! {:?} replaces {:?}",
                                   x,
                                   y,
                                   (pos, key),
                                   replaced_value);
                        }
                    }
                }
            });
        TileMap {
            pos: level.pos,
            dimens: level.dimens,
            tiles,
            tile_defs,
        }
    }

    pub fn get_tile(&self, pos: &Pos) -> Option<&TileDefinition> {
        self.tiles
            .get(pos)
            .map(|tile| match tile {
                Tile::Dummy(pos) => match self.tiles.get(pos) {
                    Some(Tile::TileDefKey(key)) => Some(key),
                    _ => {
                        error!("Error! Dummy position lookup failed for tile {:?}", tile);
                        None
                    }
                },
                Tile::TileDefKey(key) => Some(key),
                _ => None,
            })
            .flatten()
            .map(|tile_def_key| self.tile_defs.get(tile_def_key))
    }

    pub fn is_tile_def_key(&self, pos: &Pos) -> bool {
        if let Some(Tile::TileDefKey(_)) = self.tiles.get(pos) {
            true
        } else {
            false
        }
    }

    pub fn remove_tile(&mut self, pos: &Pos) -> bool {
        self.tiles.remove(pos).is_some()
    }

    pub fn put_tile(&mut self, pos: Pos, tile_def_key: String) {
        self.tiles.insert(pos, Tile::TileDefKey(tile_def_key));
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Tile {
    /// A dummy tile, points towards its anchor point, where the real tile is stored.
    /// Dummy tiles are used when a tile is bigger than 1 by 1. The bottom-left position within the
    /// tile will be set as the tile itself. All other positions will contain a dummy that points
    /// towards that bottom-left position.
    Dummy(Pos),
    /// A tile. Contains a tile definition key.
    TileDefKey(String),
    /// Normally, if there is no mapping for a position, that position counts as occupied by
    /// an air block. Therefore, Tile::AirBlock is not used during normal game play.
    ///
    /// However, if you specifically want to override an existing block with an air block,
    /// you can use this. Currently, this only happens in the level editor.
    AirBlock,
}

impl Tile {
    /// Returns true iff the enum is an actual tile, rather than an air block or a dummy reference.
    pub fn is_tile_def(&self) -> bool {
        if let Tile::TileDefKey(_) = self {
            true
        } else {
            false
        }
    }
}
