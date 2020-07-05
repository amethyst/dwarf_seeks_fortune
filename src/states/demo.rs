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
use crate::game_data::CustomGameData;
use crate::levels::*;
use crate::resources::*;
use crate::states::PausedState;

pub struct DemoState {
    fps_ui: Handle<UiPrefab>,
    paused_ui: Handle<UiPrefab>,
}

impl<'a, 'b> DemoState {
    pub fn new(fps_ui: Handle<UiPrefab>, paused_ui: Handle<UiPrefab>) -> DemoState {
        DemoState { fps_ui, paused_ui }
    }

    fn handle_action(
        &mut self,
        action: &str,
        world: &mut World,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        let mut config = world.fetch_mut::<DebugConfig>();
        if action == "speedUp" {
            let (old_speed, new_speed) = (*config).increase_speed();
            println!("Speeding up, from {:?} to {:?}", old_speed, new_speed);
            Trans::None
        } else if action == "slowDown" {
            let (old_speed, new_speed) = (*config).decrease_speed();
            println!("Slowing down, from {:?} to {:?}", old_speed, new_speed);
            Trans::None
        } else {
            Trans::None
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for DemoState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        let StateData { world, .. } = data;
        let background = world
            .read_resource::<Assets>()
            .get_still(&SpriteType::Background);
        initialise_camera(world);
        setup_debug_lines(world);
        world.write_resource::<History>().force_key_frame = true;
        initialize_sprite(world, background);
        load_level(world);
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
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    // Pause the game by going to the `PausedState`.
                    Trans::Push(Box::new(PausedState::new(
                        data.world
                            .create_entity()
                            .with(self.paused_ui.clone())
                            .build(),
                    )))
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

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.fetch::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    let camera_frame = world
        .create_entity()
        .with(CameraFrameTag::default())
        .with(transform)
        .build();

    world
        .create_entity()
        .with(Parent {
            entity: camera_frame,
        })
        .with(Camera::standard_2d(width, height))
        .with(Transform::default())
        .build();
}

/// Background init. Background is temporary, just to test out tinting when rewinding.
fn initialize_sprite(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    // Move the sprite to the middle of the window
    let mut sprite_transform = Transform::default();
    sprite_transform.set_translation_xyz(width / 2., height / 2., -1.);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // First sprite
    };

    // White shows the sprite as normal.
    // You can change the color at any point to modify the sprite's tint.
    let tint = Tint(Srgba::new(1.0, 1.0, 1.0, 1.0));

    world
        .create_entity()
        .with(sprite_render)
        .with(sprite_transform)
        .with(tint)
        // .with(Transparent) // If your sprite is transparent
        .build();
}
