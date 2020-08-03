use crate::levels::ToolType;
use amethyst::{
    assets::PrefabData,
    core::math::Vector2,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, NullStorage, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

/// The entity with this component is the player.
#[derive(Clone, Copy, Debug, Component, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Player {
    pub equipped: Option<ToolType>,
}

/// The entity with this component is a tool equipped by the player.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct EquippedTag;

impl Component for EquippedTag {
    type Storage = NullStorage<Self>;
}

/// A debug entity that shows the player's current discrete position.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct DebugPosGhostTag;

impl Component for DebugPosGhostTag {
    type Storage = NullStorage<Self>;
}

/// A debug entity that shows the player's destination.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct DebugSteeringGhostTag;

impl Component for DebugSteeringGhostTag {
    type Storage = NullStorage<Self>;
}
