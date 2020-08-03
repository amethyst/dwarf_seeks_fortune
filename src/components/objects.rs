use crate::levels::ToolType;
use amethyst::{
    assets::PrefabData,
    core::math::Vector2,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, NullStorage, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct KeyTag;

impl Component for KeyTag {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Tool {
    pub tool_type: ToolType,
}

impl Tool {
    pub fn new(tool_type: ToolType) -> Self {
        Tool { tool_type }
    }
}
