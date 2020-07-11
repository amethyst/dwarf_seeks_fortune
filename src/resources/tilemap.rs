use crate::components::*;
use crate::levels::*;

#[derive(Debug, Default)]
pub struct TileMap {
    level: Level,
    tile_defs: TileDefinitions,
}

impl TileMap {
    pub fn new(level: Level, tile_defs: TileDefinitions) -> Self {
        TileMap { level, tile_defs }
    }
    pub fn get_tile(&self, pos: &Pos) -> Option<&TileDefinition> {
        self.level
            .tile_defs
            .get(pos)
            .map(|key| self.tile_defs.get(key))
    }
}
