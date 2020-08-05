use crate::components::Pos;
use crate::levels::ToolType;
use crate::resources::{AssetType, SpriteType};
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
    pub sprite: SpriteType,
    pub sprite_nr: usize,
}

impl Tool {
    pub fn new(tool_type: ToolType, sprite: SpriteType, sprite_nr: usize) -> Self {
        Tool {
            tool_type,
            sprite,
            sprite_nr,
        }
    }
}

/// All destructible entities must have this component, this is how we find and delete them.
#[derive(Clone, Copy, Debug, Default, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Block {
    pub pos: Pos,
}
