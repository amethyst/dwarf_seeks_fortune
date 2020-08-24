use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, NullStorage, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

use crate::levels::ToolType;
use amethyst::core::ecs::HashMapStorage;

/// The entity with this component is the player.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Player {
    /// The tool currently equipped by the player.
    pub equipped: Option<ToolType>,
    /// Whether the jump key is currently down. Needed to figure out if the player wants to jump
    /// this frame. (Jump is only executed if this value changes from false to true.)
    pub pressing_jump: bool,
    /// How many seconds have passed since the character started jumping?
    ///
    /// This value is usually None. When the character starts jumping, it is assigned Some(0.0).
    /// The delta_seconds is added to this value every tick. Once it surpasses a threshold, it is
    /// set back to None.
    ///
    /// As long as the grace timer hasn't run out yet, the player can give their jump horizontal
    /// speed. This fixes the problem that if the player presses jump and move at the same time,
    /// jump is sometimes registered before move and the character only jumps up, not sideways.
    pub jump_grace_timer: Option<f32>,
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
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
