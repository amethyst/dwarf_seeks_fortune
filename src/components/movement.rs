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
    /// The entity is falling straight down.
    Falling {
        /// The x-movement that the entity has while falling. This will remain constant.
        /// It is either a -1 (move to left) 0 (don't move along x-axis) or 1 (move right).
        x_movement: f32,
        /// The y-coordinate that the entity had when it  first started falling.
        starting_y_pos: f32,
        /// A time in seconds since the start of the game. This is the time at which the entity
        /// started falling.
        starting_time: f64,
    },
    /// The entity is jumping. The character may have an x-velocity.
    /// While jumping, the character's y-coordinate describes a certain curve.
    /// This also takes the original y-coordinate and the start time.
    /// These are necessary to be able to interpolate the y-coordinate.
    Jumping {
        /// The x-movement that the entity has while jumping. This will remain constant.
        /// It is either a -1 (move to left) 0 (don't move along x-axis) or 1 (move right).
        x_movement: f32,
        /// The y-coordinate that the entity had when it started the jump.
        starting_y_pos: f32,
        /// A time in seconds since the start of the game. This is the time at which the entity
        /// started jumping.
        starting_time: f64,
    },
}

impl Default for SteeringMode {
    fn default() -> Self {
        SteeringMode::Grounded
    }
}

impl SteeringMode {
    /// Calculate the y offset from the initial y-position at the time this movement began.
    /// This method is only valid for SteeringMode::Falling and SteeringMode::Jumping. It will
    /// return 0. otherwise.
    pub fn calc_delta_y(&self, time: f64) -> f32 {
        match self {
            SteeringMode::Jumping {
                starting_y_pos,
                starting_time,
                ..
            } => {
                let t = time - starting_time;
                (-50. * (t - 0.209).powf(2.) + 2.2) as f32
            }
            SteeringMode::Falling {
                starting_y_pos,
                starting_time,
                ..
            } => {
                let t = time - starting_time;
                (t * -15.) as f32
            }
            _ => 0.,
        }
    }

    pub fn jump_to_fall(&self) -> Self {
        if let SteeringMode::Jumping {
            x_movement,
            starting_y_pos,
            starting_time,
        } = *self
        {
            SteeringMode::Falling {
                x_movement,
                starting_y_pos: starting_y_pos + self.calc_delta_y(starting_time + 0.209),
                starting_time: starting_time + 0.209,
            }
        } else {
            panic!("Not allowed.");
        }
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
        match self.mode {
            SteeringMode::Falling { .. } => true,
            SteeringMode::Jumping { .. } => true,
            _ => false,
        }
    }

    pub fn is_jumping(&self) -> bool {
        if let SteeringMode::Jumping { .. } = self.mode {
            true
        } else {
            false
        }
    }

    pub fn jump_has_peaked(&self, time: f64) -> bool {
        if let SteeringMode::Jumping {
            starting_time: start_time,
            ..
        } = self.mode
        {
            time - start_time > 0.209
        } else {
            false
        }
    }

    pub fn is_falling(&self) -> bool {
        if let SteeringMode::Falling { .. } = self.mode {
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
