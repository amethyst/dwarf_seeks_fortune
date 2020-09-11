use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, NullStorage, WriteStorage},
    error::Error,
};
use dsf_core::components::Direction2D;
use serde::{Deserialize, Serialize};

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
    /// Time in seconds before cursor is allowed to move again.
    pub movement_cooldown: f32,
    pub is_visible: bool,
    /// Time in seconds before cursor is allowed to change its visibility, as part of its
    /// blinking animation. This will be reset when the cursor moves, so as not to obscure the
    /// cursor when the user is actually moving it.
    pub blink_cooldown: f32,
}
