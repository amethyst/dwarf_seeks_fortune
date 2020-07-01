use crate::components::*;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

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