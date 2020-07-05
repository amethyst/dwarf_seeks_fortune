use crate::components::*;
use crate::resources::*;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

pub struct CursorSystem {
    cooldown: f32,
}

impl Default for CursorSystem {
    fn default() -> Self {
        CursorSystem { cooldown: 0.0 }
    }
}

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, DiscretePos>,
        ReadStorage<'s, CursorTag>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut transforms, mut discrete_positions, cursor_tags, input, time): Self::SystemData,
    ) {
        for (_, transform, discrete_pos) in
        (&cursor_tags, &mut transforms, &mut discrete_positions).join()
        {
            if self.cooldown.is_sign_positive() {
                self.cooldown -= time.delta_seconds();
                return;
            }
            //TODO: Rewrite this, this is really shit.
            let input_x = input.axis_value("move_x").unwrap_or(0.0);
            let input_y = input.axis_value("move_y").unwrap_or(0.0);
            if input_x.abs() > f32::EPSILON {
                discrete_pos.x += input_x as i32;
                self.cooldown = 0.2;
            }
            if input_y.abs() > f32::EPSILON {
                discrete_pos.y += input_y as i32;
                self.cooldown = 0.2;
            }
            transform.set_translation_xyz(discrete_pos.x as f32 * 50. + 25., discrete_pos.y as f32 * 50. + 25., 0.0);
        }
    }
}
