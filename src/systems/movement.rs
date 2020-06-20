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
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Player>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(
        &mut self,
        (mut transforms, mut velocities, mut players, time, input, screen_dimens): Self::SystemData,
    ) {
        for (transform, velocity) in (&mut transforms, &mut velocities).join() {
            transform.set_translation_x(
                (transform.translation().x + time.delta_seconds() * velocity.x)
                    .min(screen_dimens.width() - MOB_RADIUS * 0.5)
                    .max(MOB_RADIUS * 0.5),
            );
            transform.set_translation_y(
                (transform.translation().y + time.delta_seconds() * velocity.y)
                    .min(screen_dimens.height() - MOB_RADIUS * 0.5)
                    .max(MOB_RADIUS * 0.5),
            );
        }

        for (player, transform) in (&mut players, &mut transforms).join() {
            let x_axis = input.axis_value("move_x");
            let y_axis = input.axis_value("move_y");
            if let Some(signum) = x_axis {
                player.velocity.x = (SHIP_FRICTION
                    * (player.velocity.x
                        + signum * time.delta_seconds() * SHIP_ACCELERATION[0]))
                    .max(-MOB_MAX_SPEED)
                    .min(MOB_MAX_SPEED);
                transform.set_translation_x(
                    (transform.translation().x
                        + time.delta_seconds() * player.velocity.x)
                        .min(screen_dimens.width() - MOB_RADIUS * 0.5)
                        .max(MOB_RADIUS * 0.5),
                );
            }
            if let Some(signum) = y_axis {
                player.velocity.y = (SHIP_FRICTION
                    * (player.velocity.y
                        + signum * time.delta_seconds() * SHIP_ACCELERATION[1]
                        + time.delta_seconds() * SHIP_GRAVITY))
                    .max(-MOB_MAX_SPEED)
                    .min(MOB_MAX_SPEED);
                transform.set_translation_y(
                    (transform.translation().y
                        + time.delta_seconds() * player.velocity.y)
                        .min(screen_dimens.height() - MOB_RADIUS * 0.5)
                        .max(MOB_RADIUS * 0.5),
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
