use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, NullStorage, VecStorage, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity { x, y }
    }
}

#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Health {
    hp: u32,
    max_hp: u32,
}

impl Health {
    pub fn new(max_hp: u32) -> Health {
        Health { hp: max_hp, max_hp }
    }
}

// Deprecated: Get rid of player in this form.
#[derive(Debug, Deserialize, Serialize, PrefabData)]
pub struct Player {
    pub velocity: Velocity,
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct DebugOrbTag;

impl Component for DebugOrbTag {
    type Storage = NullStorage<Self>;
}
