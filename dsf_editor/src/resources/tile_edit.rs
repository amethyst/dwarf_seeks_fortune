#[derive(Debug, Clone, Default)]
pub struct TileEdit {
    pub tile_def_key: String,
    pub dirty: bool,
}

impl TileEdit {
    #[must_use]
    pub fn new(tile_def_key: String) -> Self {
        TileEdit {
            tile_def_key,
            dirty: true,
        }
    }
}
