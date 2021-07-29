use crate::components::Pos;
use serde::{Deserialize, Serialize};

/// The minimum dimension (both horizontal and vertical) that the world bounds should have.
/// No level can ever be smaller than a 2 by 2.
///
/// Any dimension lower than this would break the editor:
/// - If a dimension is zero, there wouldn't be any room for a cursor to exist.
/// - If a dimension is one, you couldn't expand one border without contracting the other border.
const MIN_DIMENSION: i32 = 2;

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
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
    #[must_use]
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        WorldBounds {
            pos: Pos::new(x, y),
            dimens: Pos::new(width, height),
        }
    }

    /// Inclusive lower bound.
    #[must_use]
    pub fn x(&self) -> i32 {
        self.pos.x
    }

    /// Inclusive lower bound.
    #[must_use]
    pub fn y(&self) -> i32 {
        self.pos.y
    }

    #[must_use]
    pub fn width(&self) -> i32 {
        self.dimens.x
    }

    #[must_use]
    pub fn height(&self) -> i32 {
        self.dimens.y
    }

    /// Exclusive upper bound.
    #[must_use]
    pub fn upper_x(&self) -> i32 {
        self.pos.x + self.dimens.x
    }

    /// Exclusive upper bound.
    #[must_use]
    pub fn upper_y(&self) -> i32 {
        self.pos.y + self.dimens.y
    }

    /// Clamp the given position inside the world bounds.
    /// The resulting position is always inside the world.
    #[must_use]
    pub fn clamp(&self, pos: Pos) -> Pos {
        Pos::new(self.clamp_x(pos.x), self.clamp_y(pos.y))
    }

    /// Clamp the given x-coordinate inside the world bounds.
    /// The resulting coordinate is always inside the world.
    fn clamp_x(&self, x: i32) -> i32 {
        if x < self.x() {
            self.x()
        } else if x >= self.upper_x() {
            self.upper_x() - 1
        } else {
            x
        }
    }

    /// Clamp the given y-coordinate inside the world bounds.
    /// The resulting coordinate is always inside the world.
    fn clamp_y(&self, y: i32) -> i32 {
        if y < self.y() {
            self.y()
        } else if y >= self.upper_y() {
            self.upper_y() - 1
        } else {
            y
        }
    }

    /// Checks if the rectangle at the given position with the given dimensions is completely
    /// enclosed within the world bounds.
    /// Can be used to check if a tile can be placed in the world.
    /// If it's (partially) out of bounds, this method will return false.
    #[must_use]
    pub fn encloses(&self, pos: Pos, dimensions: Pos) -> bool {
        self.x() <= pos.x
            && self.y() <= pos.y
            && self.upper_x() >= pos.x + dimensions.x
            && self.upper_y() >= pos.y + dimensions.y
    }

    /// This is how to adjust the horizontal borders of the level.
    /// `from` is the x-coordinate of one of the borders. If it is not a border, nothing will happen.
    /// `delta` is by how much to adjust that border. The borders can never be adjusted to the
    /// point where the dimensions of the level drop below 2 by 2.
    pub fn adjust_x(&mut self, from: i32, delta: i32) {
        if from == self.x() && self.dimens.x - delta >= MIN_DIMENSION {
            self.pos.x += delta;
            self.dimens.x -= delta;
        } else if from == self.upper_x() - 1 && self.dimens.x + delta >= MIN_DIMENSION {
            self.dimens.x += delta;
        }
    }

    /// This is how to adjust the vertical borders of the level.
    /// `from` is the y-coordinate of one of the borders. If it is not a border, nothing will happen.
    /// `delta` is by how much to adjust that border. The borders can never be adjusted to the
    /// point where the dimensions of the level drop below 2 by 2.
    pub fn adjust_y(&mut self, from: i32, delta: i32) {
        if from == self.y() && self.dimens.y - delta >= MIN_DIMENSION {
            self.pos.y += delta;
            self.dimens.y -= delta;
        } else if from == self.upper_y() - 1 && self.dimens.y + delta >= MIN_DIMENSION {
            self.dimens.y += delta;
        }
    }
}
