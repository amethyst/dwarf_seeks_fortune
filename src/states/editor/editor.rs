use amethyst::prelude::WorldExt;
use amethyst::ui::UiPrefab;
use amethyst::State;
use amethyst::StateEvent;
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    assets::{AssetStorage, Handle, Loader, Prefab},
    core::{
        math::{Point2, Vector3},
        transform::Transform,
        Parent,
    },
    ecs::{prelude::World, Entities, Entity, Join, ReadStorage, WriteStorage},
    input::{get_key, is_close_requested, is_key_down, InputEvent, VirtualKeyCode},
    prelude::*,
    renderer::{
        formats::texture::ImageFormat, palette::Srgba, resources::Tint, sprite::SpriteRender,
        Camera, SpriteSheet, Texture, Transparent,
    },
    utils::application_root_dir,
    window::ScreenDimensions,
    winit::{Event, WindowEvent},
    StateData, Trans,
};
use precompile::AnimationId;

use crate::components::*;
use crate::entities::*;
use crate::game_data::CustomGameData;
use crate::levels::*;
use crate::resources::*;
use crate::states::editor::file_actions::{auto_save, auto_save_file, load_auto_save, save};
use crate::states::editor::paint::erase_tiles;
use crate::states::editor::paint::paint_tiles;
use crate::states::{window_event_handler, PausedState, PlayState};
use std::io::Read;

pub struct EditorState;

impl<'a, 'b> EditorState {
    fn handle_action(
        &mut self,
        action: &str,
        world: &mut World,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        Trans::None
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for EditorState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        info!("EditorState on_start");
        let StateData { world, .. } = data;
        UiHandles::add_ui(&UiType::Fps, world);
        UiHandles::add_ui(&UiType::Editor, world);
        setup_debug_lines(world);
        init_cursor(world);
        create_camera(world);
        let mut editor_data = EditorData::default();
        if let Ok(level_edit) = load_auto_save() {
            editor_data.level = level_edit;
        }
        let tile_defs = load_tile_definitions().expect("Tile definitions failed to load!");
        editor_data.brush.set_palette(&tile_defs);
        world.insert(editor_data);
        world.insert(tile_defs);
    }

    fn on_stop(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        info!("EditorState on_stop");
        data.world.delete_all();
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        window_event_handler::handle(&event, data.world);
        match event {
            // Events related to the window and inputs.
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    auto_save(data.world);
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            // Ui event. Button presses, mouse hover, etc...
            StateEvent::Ui(_) => Trans::None,
            StateEvent::Input(input_event) => match input_event {
                InputEvent::KeyReleased {
                    key_code: VirtualKeyCode::Return,
                    scancode: _,
                } => {
                    paint_tiles(data.world);
                    Trans::None
                }
                InputEvent::KeyReleased {
                    key_code: VirtualKeyCode::Delete,
                    scancode: _,
                } => {
                    erase_tiles(data.world);
                    Trans::None
                }
                InputEvent::KeyReleased {
                    key_code: VirtualKeyCode::F1,
                    scancode: _,
                } => {
                    auto_save(data.world);
                    Trans::Switch(Box::new(PlayState::new(auto_save_file(), true)))
                }
                InputEvent::KeyReleased {
                    key_code: VirtualKeyCode::LBracket,
                    scancode: _,
                } => {
                    let new_key = (*data.world.write_resource::<EditorData>())
                        .brush
                        .select_previous();
                    add_cursor_preview_tag(data.world, new_key);
                    Trans::None
                }
                InputEvent::KeyReleased {
                    key_code: VirtualKeyCode::RBracket,
                    scancode: _,
                } => {
                    let new_key = (*data.world.write_resource::<EditorData>())
                        .brush
                        .select_next();
                    add_cursor_preview_tag(data.world, new_key);
                    Trans::None
                }
                InputEvent::ActionPressed(action) => {
                    self.handle_action(&action, data.world);
                    Trans::None
                }
                _ => Trans::None,
            },
        }
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        let StateData { world, .. } = data;
        // Execute a pass similar to a system
        world.exec(
            |(entities, animation_sets, mut control_sets): (
                Entities,
                ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
                WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
            )| {
                // For each entity that has AnimationSet
                for (entity, animation_set) in (&entities, &animation_sets).join() {
                    // Creates a new AnimationControlSet for the entity
                    let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                    // Adds the `Fly` animation to AnimationControlSet and loops infinitely
                    control_set.add_animation(
                        AnimationId::Fly,
                        &animation_set.get(&AnimationId::Fly).unwrap(),
                        EndControl::Loop(None),
                        1.0,
                        AnimationCommand::Start,
                    );
                }
            },
        );
        data.data.update(&world, true);
        Trans::None
    }
}

fn init_cursor(world: &mut World) {
    let sprite_handle = world
        .read_resource::<Assets>()
        .get_still(&SpriteType::Selection);
    let asset_dimensions = get_asset_dimensions(&AssetType::Still(SpriteType::Selection, 0));
    let mut selection_transform = Transform::default();
    selection_transform.set_translation_z((&DepthLayer::UiElements).z());
    world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: sprite_handle.clone(),
            sprite_number: 1,
        })
        .with(Transparent)
        .with(selection_transform)
        .with(SelectionTag)
        .build();
    let mut transform = Transform::default();
    // transform.set_translation_xyz(0.5, 0.5, 2.0);
    let cursor = world
        .create_entity()
        .with(transform)
        .with(Cursor::default())
        .build();
    add_cursor_preview_tag(world, None);
}

//TODO: Very crappy code.
fn add_cursor_preview_tag(world: &mut World, key: Option<String>) {
    let cursor = lookup_cursor_entity(world);
    delete_cursor_preview(world);
    if key.is_none() {
        let sprite_sheet = world
            .read_resource::<Assets>()
            .get_still(&SpriteType::Selection);
        let asset_dimensions = get_asset_dimensions(&AssetType::Still(SpriteType::Selection, 2));
        let mut transform = Transform::default();
        transform.set_translation_xyz(0.5, 0.5, (&DepthLayer::UiElements).z());
        transform.set_scale(Vector3::new(
            1. / asset_dimensions.x as f32,
            1. / asset_dimensions.y as f32,
            1.0,
        ));
        let parent = world
            .create_entity()
            .with(CursorPreviewParentTag)
            .with(Parent { entity: cursor })
            .with(transform)
            .build();
        world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet,
                sprite_number: 2,
            })
            .with(Transparent)
            .with(Transform::default())
            .with(CursorPreviewTag)
            .with(Parent { entity: parent })
            .build();
    } else {
        let tile_def = world
            .read_resource::<TileDefinitions>()
            .get(&key.unwrap())
            .clone();
        let still_asset = load_still_asset(&tile_def, &world.read_resource::<Assets>());
        let anim_asset = load_anim_asset(&tile_def, &world.read_resource::<Assets>());
        let transform = if let Some(asset) = &tile_def.asset {
            Some(load_transform(
                &Pos::default(),
                &DepthLayer::UiElements,
                &tile_def.dimens,
                asset,
            ))
        } else {
            panic!("Not implemented yet! Tiles with no graphics asset."); //TODO;...
        };
        let parent = world
            .create_entity()
            .with(CursorPreviewParentTag)
            .with(transform.unwrap())
            .with(Parent { entity: cursor })
            .build();
        let mut builder = world.create_entity();
        if let Some(still_asset) = still_asset {
            builder = builder.with(still_asset);
        }
        if let Some(anim_asset) = anim_asset {
            builder = builder.with(anim_asset);
        }
        builder
            .with(Transform::default())
            .with(Transparent)
            .with(Tint(Srgba::new(0.4, 0.4, 0.4, 0.8)))
            .with(CursorPreviewTag)
            .with(Parent { entity: parent })
            .build();
    }
}

fn lookup_cursor_entity(world: &mut World) -> Entity {
    world.exec(|mut data: (ReadStorage<Cursor>, Entities)| {
        let (cursors, entities) = data;
        let (entity, _) = (&entities, &cursors)
            .join()
            .next()
            .expect("Help! Cursor entity does not exist!");
        entity
    })
}

fn delete_cursor_preview(world: &mut World) {
    world.exec(
        |mut data: (ReadStorage<CursorPreviewParentTag>, Entities)| {
            let (previews, entities) = data;
            (&entities, &previews)
                .join()
                .map(|(entity, _)| entity)
                .for_each(|preview| {
                    entities.delete(preview);
                });
        },
    );
}
