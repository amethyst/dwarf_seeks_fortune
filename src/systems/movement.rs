use crate::components::*;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

const MOB_RADIUS: f32 = 32.0;
const MOB_MAX_SPEED: f32 = 500.0;
const SHIP_ACCELERATION: [f32; 2] = [500.0, 500.0];
const SHIP_FRICTION: f32 = 1.0;
const SHIP_GRAVITY: f32 = 0.0;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut transforms, velocities, time): Self::SystemData,
    ) {
        for (transform, velocity) in (&mut transforms, &velocities).join() {
            transform.set_translation_x(
                (transform.translation().x + time.delta_seconds() * velocity.x),
            );
            transform.set_translation_y(
                (transform.translation().y + time.delta_seconds() * velocity.y),
            );
        }
    }
}

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, DiscretePos>,
        WriteStorage<'s, Steering>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, PlayerTag>,
        ReadStorage<'s, PlayerDebugGhostTag>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (mut transforms, mut discrete_positions, mut steerings, mut velocities, player_tags, ghost_tags, time, input): Self::SystemData,
    ) {
        // let delta_time = time.delta_seconds();
        let mut destination = None;
        for (_, transform, discrete_pos, steering, velocity) in (&player_tags, &transforms, &mut discrete_positions, &mut steerings, &mut velocities).join() {
            let base_pos = transform.translation().x / 50.0;
            let pos_remainder = transform.translation().x % 50.0; //FIXME: probably breaks when using negative numbers.
            discrete_pos.x = if pos_remainder > 25.0 {
                base_pos
            } else {
                base_pos - 1.0
            } as i32;
            println!("base_pos={:?} remainder: {:?} discrete: {:?}", base_pos, pos_remainder, discrete_pos.x);

            let x_axis = input.axis_value("move_x");
            // let y_axis = input.axis_value("move_y");
            if let Some(signum) = x_axis {
                if signum.abs() > f32::EPSILON {
                    steering.direction = signum;
                    steering.destination.x = discrete_pos.x + signum as i32;
                }
            }
            destination = Some(steering.destination.clone());
        }
        for (_, transform, discrete_pos, steering, velocity) in (&player_tags, &mut transforms, &discrete_positions, &steerings, &mut velocities).join() {
            let desired_pos = steering.destination.x as f32 * 50.0 + 50.0;
            let delta = desired_pos - transform.translation().x;
            let signum =
                if delta.abs() < f32::EPSILON {
                    0.0
                } else if delta.is_sign_positive() {
                    1.0
                } else {
                    -1.0
                };
            if (delta * steering.direction).is_sign_positive() {
                velocity.x = signum * 200.0;
            } else {
                velocity.x = 0.0;
            }
        }
        if let Some(destination) = destination {
            for (_, transform) in (&ghost_tags, &mut transforms).join() {
                transform.set_translation_x(
                    (destination.x * 50 + 50) as f32,
                );
                transform.set_translation_y(
                    (destination.y * 50 + 50) as f32,
                );
            }
        }
    }
}

pub struct DebugSystem;

impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, DebugOrbTag>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut transforms, debug_orbs, input, screen_dimens): Self::SystemData) {
        for (transform, debug_orb) in (&mut transforms, &debug_orbs).join() {
            let x_axis = input.axis_value("move_x");
            let y_axis = input.axis_value("move_y");
            if let Some(signum) = x_axis {
                if signum.abs() > 0.01 {
                    println!("Move x signum={:?}\t dimens:{:?}", signum, screen_dimens.width());
                    transform.set_translation_x((screen_dimens.width() * signum).max(0.0));
                }
            }
            if let Some(signum) = y_axis {
                if signum.abs() > 0.01 {
                    println!("Move y signum={:?}\t dimens:{:?}", signum, screen_dimens.height());
                    transform.set_translation_y((screen_dimens.height() * signum).max(0.0));
                }
            }
        }
    }
}
