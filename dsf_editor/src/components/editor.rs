use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, NullStorage, WriteStorage},
    error::Error,
};
use dsf_core::components::{Direction2D, Pos};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct SelectionTag;

impl Component for SelectionTag {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct CursorPreviewParentTag;

impl Component for CursorPreviewParentTag {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct CursorPreviewTag;

impl Component for CursorPreviewTag {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Cursor {
    pub last_direction: Direction2D,
    pub cooldown: f32,
}

#[derive(Clone, Copy, Debug, Default, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct PaintedTile {
    pub pos: Pos,
}

impl PaintedTile {
    pub fn new(pos: Pos) -> Self {
        PaintedTile { pos }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct EditorRootTag;

impl Component for EditorRootTag {
    type Storage = NullStorage<Self>;
}
