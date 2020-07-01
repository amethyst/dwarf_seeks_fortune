use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, NullStorage, VecStorage, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct DebugConfig {
    pub player_speed: f32,
}

impl Default for DebugConfig {
    fn default() -> Self {
        DebugConfig {
            player_speed: 200.0,
        }
    }
}