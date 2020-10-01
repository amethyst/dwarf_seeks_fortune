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

    pub fn clamp(&self, pos: &Pos) -> Pos {
        Pos::new(self.clamp_x(pos.x), self.clamp_y(pos.y))
    }

    fn clamp_x(&self, x: i32) -> i32 {
        if x < self.x() {
            self.x()
        } else if x >= self.upper_x() {
            self.upper_x() - 1
        } else {
            x
        }
    }

    fn clamp_y(&self, y: i32) -> i32 {
        if y < self.y() {
            self.y()
        } else if y >= self.upper_y() {
            self.upper_y() - 1
        } else {
            y
        }
    }
}
