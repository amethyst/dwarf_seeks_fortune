use amethyst::{
    assets::PrefabData,
    core::math::Vector2,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, NullStorage, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

/// Velocity in meters per second.
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

/// A discrete position in the world, with x and y being integral numbers.
/// Used among other things for positioning tiles, which are always snapped to the grid.
///
/// Not to be confused with Transform, which contains an entity's actual position.
#[derive(
    Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData, PartialEq, Eq, Hash,
)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

/// Remembers the direction an entity is moving in. Also keeps a destination as a discrete position.
/// Steering is used to accomplish the snap-to-grid, tile-based movement.
#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Steering {
    pub direction: f32,
    pub destination: Pos,
    pub grounded: bool,
}

impl Steering {
    pub fn new(destination: Pos) -> Steering {
        Steering {
            direction: 0.0,
            destination,
            grounded: true,
        }
    }
}

#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Movement {
    pub grounded: bool,
}

// #[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
// #[prefab(Component)]
// #[serde(deny_unknown_fields)]
// pub struct Collider {
//     pub collides_top: bool,
//     pub collides_rest: bool,
// }
