use amethyst::core::math::Vector3;
use amethyst::prelude::WorldExt;
use amethyst::ui::UiPrefab;
use amethyst::State;
use amethyst::StateEvent;
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    assets::{AssetStorage, Handle, Loader, Prefab},
    core::{transform::Transform, Parent},
    ecs::{prelude::World, Entities, Entity, Join, ReadStorage, WriteStorage},
    input::{get_key, is_close_requested, is_key_down, InputEvent, VirtualKeyCode},
    prelude::*,
    renderer::{
        formats::texture::ImageFormat, palette::Srgba, resources::Tint, sprite::SpriteRender,
        Camera, SpriteSheet, Texture,
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
use crate::states::PausedState;

/// TODO:
/// Have a map in resources, this is what is being built up and what will be saved later.
/// Cursor points at location. Derives width and height from block loaded on brush??
/// Brush (contains block-type)
/// When holding shift, should do multi-select (ideally only if block allows it)
/// Camera follows cursor?
pub struct EditorState {
    fps_ui: Handle<UiPrefab>,
}

impl<'a, 'b> EditorState {
    pub fn new(fps_ui: Handle<UiPrefab>) -> EditorState {
        EditorState { fps_ui }
    }

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
        let StateData { world, .. } = data;
        setup_debug_lines(world);
        let cursor = init_cursor(world);
        // create_camera_under_parent(world, cursor);
        create_camera(world);
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

    fn handle_event(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        match event {
            // Events related to the window and inputs.
            StateEvent::Window(event) => {
                if let Event::WindowEvent {
                    window_id: _,
                    event: WindowEvent::Resized(_),
                } = event
                {
                    *data.world.write_resource::<ResizeState>() = ResizeState::Resizing;
                };
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::F1) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            // Ui event. Button presses, mouse hover, etc...
            StateEvent::Ui(_) => Trans::None,
            StateEvent::Input(input_event) => {
                // println!("Input event detected! {:?}", input_event);
                if let InputEvent::ActionPressed(action) = input_event {
                    self.handle_action(&action, data.world)
                } else {
                    Trans::None
                }
            }
        }
    }
}

fn init_cursor(world: &mut World) -> Entity {
    let sprite_handle = world.read_resource::<Assets>().get_still(&SpriteType::Frame);
    world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: sprite_handle,
            sprite_number: 0,
        })
        .with(Transform::default())
        .with(DiscretePos::default())
        .with(CursorTag)
        .build()
}
