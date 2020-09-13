use crate::resources::EditorData;
use crate::systems::RefreshPreviewsEvent;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Read, System, Write};
use amethyst::input::{InputHandler, StringBindings};
use dsf_core::resources::{SignalEdge, SignalEdgeDetector};

/// Responsible for changing transient configurations for the editor. These settings stay alive
/// as long as the EditorState lives.
///
/// Currently, this system is responsible for:
///
/// - Changing what tile is on the brush.
/// - Toggling the copy-air flag.
/// - Toggling the force-place flag.
///
pub struct ConfigureEditorSystem;

impl<'s> System<'s> for ConfigureEditorSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, EventChannel<RefreshPreviewsEvent>>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, SignalEdgeDetector>,
        Write<'s, EditorData>,
    );

    fn run(&mut self, (mut channel, input, mut sed, mut editor_data): Self::SystemData) {
        if let SignalEdge::Rising = sed.edge("select_previous_brush", &input) {
            let _new_key = editor_data.brush.select_previous();
            channel.single_write(RefreshPreviewsEvent);
        }
        if let SignalEdge::Rising = sed.edge("select_next_brush", &input) {
            let _new_key = editor_data.brush.select_next();
            channel.single_write(RefreshPreviewsEvent);
        }
        if let SignalEdge::Rising = sed.edge("toggle_copy_air", &input) {
            editor_data.copy_air ^= true;
            channel.single_write(RefreshPreviewsEvent);
        }
        if let SignalEdge::Rising = sed.edge("toggle_force_place", &input) {
            editor_data.force_place ^= true;
            channel.single_write(RefreshPreviewsEvent);
        }
    }
}
