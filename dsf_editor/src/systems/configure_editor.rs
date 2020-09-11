use crate::components::{Cursor, CursorPreviewParentTag, CursorPreviewTag, SelectionTag};
use crate::resources::{EditorData, LevelEdit};
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Entities, Entity, Join, LazyUpdate, Read, ReadStorage, System, Write};
use amethyst::core::{math::Vector3, Parent, Time, Transform};
use amethyst::input::{InputEvent, InputHandler, StringBindings, VirtualKeyCode};
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::{SpriteRender, Transparent};
use dsf_core::components::Pos;
use dsf_core::levels::{load_anim_asset, load_still_asset, load_transform};
use dsf_core::resources::{
    get_asset_dimensions, AssetType, Assets, DepthLayer, EventReaders, SignalEdge,
    SignalEdgeDetector, SpriteType, TileDefinitions,
};

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
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, SignalEdgeDetector>,
        Write<'s, EditorData>,
        Read<'s, LevelEdit>,
    );

    fn run(&mut self, (input, mut sed, mut editor_data, _level_edit): Self::SystemData) {
        // TODO: add cursor ghosts.
        if let SignalEdge::Rising = sed.edge("select_previous_brush", &input) {
            let _new_key = editor_data.brush.select_previous();
            // lazy.exec(|world| {
            //     add_cursor_preview_tag(world, new_key);
            // });
        }
        if let SignalEdge::Rising = sed.edge("select_next_brush", &input) {
            let _new_key = editor_data.brush.select_next();
            // lazy.exec(|world| {
            //     add_cursor_preview_tag(world, new_key);
            // });
        }
        if let SignalEdge::Rising = sed.edge("toggle_copy_air", &input) {
            editor_data.copy_air ^= true;
        }
        if let SignalEdge::Rising = sed.edge("toggle_force_place", &input) {
            editor_data.force_place ^= true;
        }
    }
}

// fn init_cursor(world: &mut World) {
//     let sprite_handle = world
//         .read_resource::<Assets>()
//         .get_still(&SpriteType::Selection);
//     let mut selection_transform = Transform::default();
//     selection_transform.set_translation_z((&DepthLayer::UiElements).z());
//     world
//         .create_entity()
//         .with(SpriteRender {
//             sprite_sheet: sprite_handle,
//             sprite_number: 1,
//         })
//         .with(Transparent)
//         .with(selection_transform)
//         .with(SelectionTag)
//         .build();
//     let transform = Transform::default();
//     let _ = world
//         .create_entity()
//         .with(transform)
//         .with(Cursor::default())
//         .build();
//     add_cursor_preview_tag(world, None);
// }
//
// //TODO: Very crappy code.
// fn add_cursor_preview_tag(world: &mut World, key: Option<String>) {
//     let cursor = lookup_cursor_entity(world);
//     delete_cursor_preview(world);
//     if let Some(key) = key {
//         let tile_def = world.read_resource::<TileDefinitions>().get(&key).clone();
//         let still_asset = load_still_asset(&tile_def, &world.read_resource::<Assets>());
//         let anim_asset = load_anim_asset(&tile_def, &world.read_resource::<Assets>());
//         let transform = if let Some(asset) = &tile_def.asset {
//             Some(load_transform(
//                 &Pos::default(),
//                 &DepthLayer::UiElements,
//                 &tile_def.dimens,
//                 asset,
//             ))
//         } else {
//             panic!("Not implemented yet! Tiles with no graphics asset."); //TODO;...
//         };
//         let parent = world
//             .create_entity()
//             .with(CursorPreviewParentTag)
//             .with(transform.unwrap())
//             .with(Parent { entity: cursor })
//             .build();
//         let mut builder = world.create_entity();
//         if let Some(still_asset) = still_asset {
//             builder = builder.with(still_asset);
//         }
//         if let Some(anim_asset) = anim_asset {
//             builder = builder.with(anim_asset);
//         }
//         builder
//             .with(Transform::default())
//             .with(Transparent)
//             .with(Tint(Srgba::new(0.4, 0.4, 0.4, 0.8)))
//             .with(CursorPreviewTag)
//             .with(Parent { entity: parent })
//             .build();
//     } else {
//         let sprite_sheet = world
//             .read_resource::<Assets>()
//             .get_still(&SpriteType::Selection);
//         let asset_dimensions = get_asset_dimensions(&AssetType::Still(SpriteType::Selection, 2));
//         let mut transform = Transform::default();
//         transform.set_translation_xyz(0.5, 0.5, (&DepthLayer::UiElements).z());
//         transform.set_scale(Vector3::new(
//             1. / asset_dimensions.x as f32,
//             1. / asset_dimensions.y as f32,
//             1.0,
//         ));
//         let parent = world
//             .create_entity()
//             .with(CursorPreviewParentTag)
//             .with(Parent { entity: cursor })
//             .with(transform)
//             .build();
//         world
//             .create_entity()
//             .with(SpriteRender {
//                 sprite_sheet,
//                 sprite_number: 2,
//             })
//             .with(Transparent)
//             .with(Transform::default())
//             .with(CursorPreviewTag)
//             .with(Parent { entity: parent })
//             .build();
//     }
// }
//
// fn lookup_cursor_entity(world: &mut World) -> Entity {
//     world.exec(|data: (ReadStorage<Cursor>, Entities)| {
//         let (cursors, entities) = data;
//         let (entity, _) = (&entities, &cursors)
//             .join()
//             .next()
//             .expect("Help! Cursor entity does not exist!");
//         entity
//     })
// }
//
// fn delete_cursor_preview(world: &mut World) {
//     world.exec(|data: (ReadStorage<CursorPreviewParentTag>, Entities)| {
//         let (previews, entities) = data;
//         (&entities, &previews)
//             .join()
//             .map(|(entity, _)| entity)
//             .for_each(|preview| {
//                 entities
//                     .delete(preview)
//                     .expect("Failed to delete CursorPreviewParentTag.");
//             });
//     });
// }
