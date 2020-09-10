use crate::components::CursorPreviewTag;
use amethyst::core::ecs::{Join, ReadStorage, System, WriteStorage};
use amethyst::core::Transform;

/// Responsible for animating the cursor preview (IE the ghost of the block on the brush
/// that is displayed at the cursor position).
pub struct CreatePreviewsSystem;

impl<'s> System<'s> for CreatePreviewsSystem {
    type SystemData = (
        ReadStorage<'s, CursorPreviewTag>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (tags, mut transforms): Self::SystemData) {
        for (_, _transform) in (&tags, &mut transforms).join() {}
    }
}
