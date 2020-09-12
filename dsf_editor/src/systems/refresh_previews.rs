use amethyst::core::ecs::{Read, ReadStorage, ReaderId, System, WriteStorage};
use amethyst::core::Transform;

use crate::components::CursorPreviewTag;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::shred::SystemData;
use amethyst::prelude::World;

/// Send this through the event bus in order to trigger a complete refresh of the previews.
#[derive(Debug, Clone)]
pub struct RefreshPreviewsEvent;

/// Responsible for refreshing the preview when it receives the signal to do so through its event
/// bus. This will add a red tint to all existing tiles that are due to be removed. It will also
/// add ghost images for all the tiles that are due to be added.
#[derive(Default)]
pub struct RefreshPreviewsSystem {
    reader_id: Option<ReaderId<RefreshPreviewsEvent>>,
}

impl<'s> System<'s> for RefreshPreviewsSystem {
    type SystemData = (
        Read<'s, EventChannel<RefreshPreviewsEvent>>,
        ReadStorage<'s, CursorPreviewTag>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (channel, tags, mut transforms): Self::SystemData) {
        let reader_id = self.reader_id.as_mut().expect(
            "`RefreshPreviewsSystem::setup` was not called before `RefreshPreviewsSystem::run`",
        );
        // We don't care how many events we received, refreshing more than once doesn't do anything.
        // Check if at least one event was received, while still making sure to empty the iterator
        // (very important, otherwise the surplus events stay in the channel until next frame).
        let at_least_one_event = channel.read(reader_id).fold(false, |_, _| true);
        if at_least_one_event {
            // TODO:
            //  - Set all of the tints.
            //  - Delete all of the ghosts.
            //  - Add the ghosts.
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        self.reader_id = Some(
            world
                .fetch_mut::<EventChannel<RefreshPreviewsEvent>>()
                .register_reader(),
        )
    }
}
