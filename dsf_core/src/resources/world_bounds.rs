use crate::components::Pos;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorldBounds {
    pub pos: Pos,
    pub dimens: Pos,
}

impl Default for WorldBounds {
    fn default() -> Self {
        WorldBounds::new(-20, -10, 40, 20)
    }
}

impl WorldBounds {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        WorldBounds {
            pos: Pos::new(x, y),
            dimens: Pos::new(width, height),
        }
    }

    pub fn x(&self) -> i32 {
        self.pos.x
    }

    pub fn y(&self) -> i32 {
        self.pos.y
    }

    pub fn width(&self) -> i32 {
        self.dimens.x
    }

    pub fn height(&self) -> i32 {
        self.dimens.y
    }

    /// Exclusive upper bound.
    pub fn upper_x(&self) -> i32 {
        self.pos.x + self.dimens.x
    }

    /// Exclusive upper bound.
    pub fn upper_y(&self) -> i32 {
        self.pos.y + self.dimens.y
    }
}
