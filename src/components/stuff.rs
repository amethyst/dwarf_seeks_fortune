use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, VecStorage, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

pub struct Ship {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Ship {
    pub fn new(velocity: [f32; 2], radius: f32) -> Ship {
        Ship { velocity, radius }
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Deserialize, Serialize, PrefabData)]
pub struct Player {
    pub velocity: Velocity,
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}
