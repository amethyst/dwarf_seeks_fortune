use crate::components::Pos;
use crate::levels::*;
use amethyst::{
    assets::PrefabData,
    core::math::Vector2,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, NullStorage, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct SelectionTag;

impl Component for SelectionTag {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Cursor {
    pub last_direction: Direction,
    pub cooldown: f32,
}

#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData, PartialEq)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Direction {
    pub hor: LineDirection,
    pub ver: LineDirection,
}

impl Direction {
    pub fn new(signum_x: f32, signum_y: f32) -> Self {
        Direction {
            hor: LineDirection::new(signum_x),
            ver: LineDirection::new(signum_y),
        }
    }

    pub fn is_opposite(&self, other: &Direction) -> bool {
        self.hor.is_opposite(&other.hor) || self.ver.is_opposite(&other.ver)
    }

    pub fn is_neutral(&self) -> bool {
        self.hor == LineDirection::Neutral && self.ver == LineDirection::Neutral
    }
}

#[derive(Clone, Copy, Component, Debug, Deserialize, Serialize, PrefabData, PartialEq)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub enum LineDirection {
    Neg,
    Pos,
    Neutral,
}

impl LineDirection {
    fn new(signum: f32) -> Self {
        if signum.abs() <= f32::EPSILON {
            LineDirection::Neutral
        } else if signum.is_sign_positive() {
            LineDirection::Pos
        } else {
            LineDirection::Neg
        }
    }
    fn is_opposite(&self, other: &LineDirection) -> bool {
        (*self == LineDirection::Neg && *other == LineDirection::Pos)
            || (*self == LineDirection::Pos && *other == LineDirection::Neg)
    }
}

impl Default for LineDirection {
    fn default() -> Self {
        LineDirection::Neutral
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct PaintedTileTag;

impl Component for PaintedTileTag {
    type Storage = NullStorage<Self>;
}
