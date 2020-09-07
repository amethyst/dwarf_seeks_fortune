use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, WriteStorage},
    error::Error,
};
use dsf_core::components::Pos;
use serde::{Deserialize, Serialize};

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
