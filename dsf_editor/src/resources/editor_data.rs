use crate::resources::{Brush, LevelEdit, Selection};

#[derive(Debug, Default)]
pub struct EditorData {
    pub level: LevelEdit,
    pub brush: Brush,
    pub selection: Selection,
}
