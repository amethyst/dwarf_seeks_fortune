use crate::components::*;
use crate::levels::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TileMap {
    pub pos: Pos,
    pub dimens: Pos,
    tiles: HashMap<Pos, Tile>,
    tile_defs: TileDefinitions,
}

#[derive(Debug)]
pub enum Tile {
    /// A dummy tile, points towards its anchor point, where the real tile is stored.
    /// Dummy tiles are used when a tile is bigger than 1 by 1. The bottom-left position within the
    /// tile will be set as the tile itself. All other positions will contain a dummy that points
    /// towards that bottom-left position.
    Dummy(Pos),
    /// A tile. Contains a tile definition key.
    Key(String),
}

impl TileMap {
    pub fn new(level: Level, tile_defs: TileDefinitions) -> Self {
        let mut tiles = HashMap::new();
        level.tiles.iter().for_each(|(pos, key)| {
            let dimens = tile_defs.get(key).dimens;
            for x in 0..dimens.x {
                for y in 0..dimens.y {
                    let tile = if x == 0 && y == 0 {
                        Tile::Key(key.clone())
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
                    Some(Tile::Key(key)) => Some(key),
                    _ => {
                        error!("Error! Dummy position lookup failed for tile {:?}", tile);
                        None
                    }
                },
                Tile::Key(key) => Some(key),
            })
            .flatten()
            .map(|tile_def_key| self.tile_defs.get(tile_def_key))
    }
    pub fn remove_tile(&mut self, pos: &Pos) {
        self.tiles.remove(pos);
    }
}
