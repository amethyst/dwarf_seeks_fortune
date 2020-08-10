use crate::components::Pos;
use crate::levels::ToolType;
use crate::resources::SpriteType;
use amethyst::core::ecs::NullStorage;
use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Key {
    pub pos: Pos,
}

impl Key {
    pub fn new(pos: Pos) -> Self {
        Key { pos }
    }
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

/// A miniature version of every key is found on the exit door.
#[derive(Clone, Copy, Debug, Default, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct KeyDisplay {
    /// The position of the corresponding key in the world. NOT the actual position of this display.
    /// The display is a miniature version of the key located somewhere on top of the door.
    pub pos: Pos,
}

impl KeyDisplay {
    pub fn new(pos: Pos) -> Self {
        KeyDisplay { pos }
    }
}

/// The exit door.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct ExitDoor;

impl Component for ExitDoor {
    type Storage = NullStorage<Self>;
}

/// The blue background sprite.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Background;

impl Component for Background {
    type Storage = NullStorage<Self>;
}
