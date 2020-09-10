use crate::resources::{Brush, DeprecatedLevelEdit, Selection};
use dsf_core::resources::TileMap;

#[derive(Debug, Default)]
pub struct DeprecatedEditorData {
    pub level: DeprecatedLevelEdit,
    pub brush: Brush,
    pub selection: Selection,
}

/// Persists through play testing. Is only reset when the EditorState goes through on_create.
pub struct EditorData {
    pub brush: Brush,
    pub selection: Selection,
    pub copy_air: bool,
    pub force_place: bool,
}

pub struct LevelEdit {
    pub tile_map: TileMap,
}
