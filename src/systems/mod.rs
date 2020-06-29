mod camera;
mod movement;
mod spawn;
mod ui;

pub use self::camera::*;
pub use self::movement::*;
pub use self::spawn::*;
pub use self::ui::*;

//TODO: Movement & Position
// Discrete: intervals of 1block
// Fluid: a float for position, actual pos on screen
// DesiredDiscrete: The desired discrete pos that entity is moving towards. Once actual pos is more than halfway onto the desiredDiscretePos, the discrete pos is updated.
// If disiredPos is same is currentPos, then if entity is still moving update the desiredpos one more block, or else transition to a standstil, where entity is just standing still.
// .
// When discrete is changed, it will trigger a game state delta to be created that tick.

//PLayer
// DiscretePos - DesiredPos - Pos(Transform) - Steering - Velocity -