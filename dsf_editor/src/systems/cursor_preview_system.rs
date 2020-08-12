use amethyst::{
    assets::{Handle, Prefab},
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::sprite::SpriteRender,
};

use crate::components::*;

use amethyst::core::num::FloatConst;

/// Responsible for animating the cursor preview (IE the ghost of the block on the brush
/// that is displayed at the cursor position).
pub struct CursorPreviewSystem;

impl<'s> System<'s> for CursorPreviewSystem {
    type SystemData = (
        ReadStorage<'s, CursorPreviewTag>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (tags, mut transforms, time): Self::SystemData) {
        for (_, transform) in (&tags, &mut transforms).join() {
            let scale_factor = 1. - 0.1 * (time.absolute_time_seconds() * f64::PI()).sin().abs();
            transform.set_scale(Vector3::new(scale_factor, scale_factor, 1.0));
        }
    }
}
