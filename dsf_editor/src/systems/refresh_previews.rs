use amethyst::core::ecs::{
    Entities, Join, LazyUpdate, Read, ReadStorage, ReaderId, System, WriteStorage,
};

use crate::components::{PaintedTile, PreviewGhostTag};
use crate::resources::{Blueprint, EditorStatus, LevelEdit};
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::shred::SystemData;
use amethyst::core::Transform;
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::resources::Tint;
use dsf_core::levels::attach_graphics;
use dsf_core::resources::{DepthLayer, Tile};

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
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'s, EventChannel<RefreshPreviewsEvent>>,
        Read<'s, EditorStatus>,
        Read<'s, LevelEdit>,
        WriteStorage<'s, Tint>,
        ReadStorage<'s, PaintedTile>,
        ReadStorage<'s, PreviewGhostTag>,
        Read<'s, LazyUpdate>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (channel, status, level_edit, mut tints, painted_tiles, previews, lazy, entities): Self::SystemData,
    ) {
        let reader_id = self.reader_id.as_mut().expect(
            "`RefreshPreviewsSystem::setup` was not called before `RefreshPreviewsSystem::run`",
        );
        // We don't care how many events we received, refreshing more than once doesn't do anything.
        // Check if at least one event was received, while still making sure to empty the iterator
        // (very important, otherwise the surplus events stay in the channel until next frame).
        let at_least_one_event = channel.read(reader_id).fold(false, |_, _| true);
        if at_least_one_event {
            let blueprint = Blueprint::from_placing_tiles(&status, &level_edit);
            let lower_bounds = status.selection.lower_bounds();
            for (tint, painted_tile) in (&mut tints, &painted_tiles).join() {
                tint.0 = if let Some(tile_def) = level_edit.tile_map.get_tile(&painted_tile.pos) {
                    if blueprint.overlaps(painted_tile.pos - lower_bounds, tile_def.dimens) {
                        Srgba::new(1., 0., 0., 1.0)
                    } else {
                        Srgba::new(1., 1., 1., 1.0)
                    }
                } else {
                    Srgba::new(1., 1., 1., 1.0)
                };
            }
            // First delete all existing previews:
            for (entity, _) in (&entities, &previews).join() {
                entities.delete(entity).expect("Failed to delete preview.");
            }
            // Then create new previews based on the current Blueprint:
            lazy.exec_mut(move |world| {
                blueprint.tiles.iter().for_each(|(blueprint_pos, tile)| {
                    if let Tile::TileDefKey(key) = tile {
                        let asset = world
                            .read_resource::<LevelEdit>()
                            .get_tile_def(key)
                            .get_preview();
                        let dimens = world.read_resource::<LevelEdit>().get_tile_def(key).dimens;
                        let mut transform = Transform::default();
                        transform.set_translation_xyz(
                            (lower_bounds.x + blueprint_pos.x) as f32 + dimens.x as f32 * 0.5,
                            (lower_bounds.y + blueprint_pos.y) as f32 + dimens.y as f32 * 0.5,
                            DepthLayer::UiElements.z(),
                        );
                        let preview = world
                            .create_entity()
                            .with(PreviewGhostTag)
                            .with(transform)
                            .build();
                        attach_graphics(
                            world,
                            preview,
                            &asset,
                            &dimens,
                            Some(Tint(Srgba::new(0.5, 0.5, 0.5, 0.7))),
                        );
                    }
                });
            });
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
