use crate::components::Player;
use crate::components::Velocity;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, System, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

const CONSTANT: f32 = 60.0;
const MOB_RADIUS: f32 = 10.0;
const MOB_MAX_SPEED: f32 = 10.0 * CONSTANT;
const SHIP_ACCELERATION: [f32; 2] = [0.5, 1.0];
const SHIP_FRICTION: f32 = 0.9;
const SHIP_GRAVITY: f32 = -0.5;

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

    fn run(&mut self, (mut transforms, mut velocities, mut players, time, input, screen_dimens): Self::SystemData) {
        for(transform, velocity) in (&mut transforms, &mut velocities) .join() {
            transform.set_translation_x(
                (transform.translation().x
                    + time.delta_seconds() * velocity.x * CONSTANT)
                    .min(screen_dimens.width() - MOB_RADIUS * 0.5)
                    .max(MOB_RADIUS * 0.5),
            );
            transform.set_translation_y(
                (transform.translation().y
                    + time.delta_seconds() * velocity.y * CONSTANT)
                    .min(screen_dimens.height() - MOB_RADIUS * 0.5)
                    .max(MOB_RADIUS * 0.5),
            );
        }

        for (player, transform) in (&mut players, &mut transforms).join() {
            // if let Some((pos_x, pos_y)) = input.mouse_position() {
            //     println!("mouse movement: x={}, y={}", pos_x, pos_y);
            //
            // }
            // println!("delta_seconds: {}", time.delta_seconds());
            let x_axis = input.axis_value("move_x");
            let y_axis = input.axis_value("move_y");
            if let Some(signum) = x_axis {
                player.velocity.x = (SHIP_FRICTION
                    * (player.velocity.x
                        + signum * time.delta_seconds() * SHIP_ACCELERATION[0] * CONSTANT))
                    .max(-MOB_MAX_SPEED)
                    .min(MOB_MAX_SPEED);
                transform.set_translation_x(
                    (transform.translation().x
                        + time.delta_seconds() * player.velocity.x * CONSTANT)
                        .min(screen_dimens.width() - MOB_RADIUS * 0.5)
                        .max(MOB_RADIUS * 0.5),
                );
            }
            if let Some(signum) = y_axis {
                player.velocity.y = (SHIP_FRICTION
                    * (player.velocity.y
                        + signum * time.delta_seconds() * SHIP_ACCELERATION[1] * CONSTANT
                        + time.delta_seconds() * SHIP_GRAVITY * CONSTANT))
                    .max(-MOB_MAX_SPEED)
                    .min(MOB_MAX_SPEED);
                transform.set_translation_y(
                    (transform.translation().y
                        + time.delta_seconds() * player.velocity.y * CONSTANT)
                        .min(screen_dimens.height() - MOB_RADIUS * 0.5)
                        .max(MOB_RADIUS * 0.5),
                );
            }
        }
    }
}
