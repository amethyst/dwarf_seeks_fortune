use amethyst::{
    assets::PrefabData,
    core::math::Vector2,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, NullStorage, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

/// Velocity in meters per second.
/// TODO: Figure out screen coords / world coords difference.
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

#[derive(
    Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData, PartialEq, Eq,
)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct DiscretePos {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Steering {
    pub direction: f32,
    pub destination: DiscretePos,
}

impl Steering {
    pub fn new(destination: DiscretePos) -> Steering {
        Steering {
            direction: 0.0,
            destination,
        }
    }
}

/// The entity with this component is the player.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct PlayerTag;

impl Component for PlayerTag {
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

/// The camera will be a child entity of the camera frame.
///
/// The camera frame will maintain the rough position of the camera. Usually this will be the
/// player's position.
///
/// The camera itself will maintain an offset position. Usually this will be at the origin
/// (no offset). If there is camera shake, that will be done through this offset.
#[derive(Clone, Copy, Debug, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct CameraFrameTag {
    /// Player will be able to pan the camera around to a limited degree.
    /// This is the current offset from the camera's default position.
    pub pan: Vector2<f32>,
    /// The maximum distance in meters that the player can pan the camera.
    pub max_pan: f32,
    /// Speed at which the camera may pan, in meters per second.
    pub panning_speed: f32,
    /// Speed at which the camera may pan back to its default position after the player lets go
    /// of the panning controls. This will be faster than the speed at which the player can pan the
    /// camera around, resulting in a sort of rubber banding effect.
    pub panning_recovery_speed: f32,
}

impl Default for CameraFrameTag {
    fn default() -> Self {
        CameraFrameTag {
            pan: Vector2::new(0., 0.),
            max_pan: 250.,
            panning_speed: 500.,
            panning_recovery_speed: 2000.,
        }
    }
}
