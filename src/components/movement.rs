use amethyst::{
    assets::PrefabData,
    core::{math::Vector2, transform::Transform},
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
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Pos {
    // TODO: consider whether to replace Pos with with nalgebra's Point2?
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
///
/// Any non-particle entity that has movement should have steering.
/// Examples of entities with steering include the Player, enemies and projectiles.
#[derive(Clone, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Steering {
    /// The entity's discrete position. Not to be confused with its Transform, which is where the
    /// entity is actually at. Transform can be between squares, the discrete position is always at
    /// a square. The discrete position has it's coordinate in integral numbers, whereas the
    /// Transform's translation is in floats.
    ///
    /// If an entity is wider than 1 by 1, the pos is the bottom-left most tile in the entity's
    /// body.
    pub pos: Pos,
    /// Width and height of the entity.
    pub dimens: Pos,
    /// Direction the player is travelling along the x-axis and y-axis.
    pub direction: (f32, f32),
    pub destination: Pos,
    pub mode: SteeringMode,
}

/// SteeringMode influences max speeds, ability to jump, ability to move, etc.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum SteeringMode {
    /// Standard mode. There is flat ground beneath the entity and the entity can either move
    /// horizontally or initiate a jump.
    Grounded,
    /// Climbing on a ladder. The entity can either climb up or down.
    Climbing,
    /// The entity is either in the middle of a jump, or is simply falling.
    MidAir(JumpCurve),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum JumpCurve {
    /// The entity is falling straight down.
    Falling,
    /// The entity is jumping. The character may have an x-velocity.
    /// While jumping, the character's y-coordinate describes a certain curve.
    /// This also takes the start time. This is necessary to be able to interpolate the
    /// y-coordinate.
    ///TODO:
    /// let t = time - start_time;
    /// 6. * (t - 0.619).powf(2.) + 2.3
    Jumping(f32),
}

impl Default for SteeringMode {
    fn default() -> Self {
        SteeringMode::Grounded
    }
}

impl Steering {
    pub fn new(pos: Pos, dimens: Pos) -> Steering {
        Steering {
            pos,
            dimens,
            direction: (0.0, 0.0),
            destination: pos,
            mode: SteeringMode::Grounded,
        }
    }

    pub fn is_grounded(&self) -> bool {
        self.mode == SteeringMode::Grounded
    }

    pub fn is_mid_air(&self) -> bool {
        if let SteeringMode::MidAir(_) = self.mode {
            true
        } else {
            false
        }
    }
    pub fn is_climbing(&self) -> bool {
        self.mode == SteeringMode::Climbing
    }

    /// Converts the given discrete position to a translation, taking into account the dimensions
    /// of the entity.
    ///
    /// The discrete position is the bottom-left corner of the entity, a translation is the
    /// center point of the entity.
    pub fn to_centered_coords(&self, pos: Pos) -> (f32, f32) {
        (
            pos.x as f32 + 0.5 * self.dimens.x as f32,
            pos.y as f32 + 0.5 * self.dimens.y as f32,
        )
    }

    /// Converts the given translation, which is the center-point of the entity, into a pair of
    /// anchored coordinates, describing the bottom-left corner of the entity.
    ///
    /// Note that this does NOT return a discrete position: output is not rounded or floored.
    pub fn to_anchor_coords(&self, transform: &Transform) -> (f32, f32) {
        (
            transform.translation().x - 0.5 * self.dimens.x as f32,
            transform.translation().y - 0.5 * self.dimens.y as f32,
        )
    }
}
