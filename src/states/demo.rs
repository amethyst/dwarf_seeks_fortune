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
    window::ScreenDimensions,
    StateData, Trans,
};

use precompile::AnimationId;

use crate::components::*;
use crate::config::*;
use crate::game_data::CustomGameData;
use crate::prefabs::Prefabs;
use crate::resources::*;
use crate::states::PausedState;

pub struct DemoState {
    prefabs: Prefabs,
    fps_ui: Handle<UiPrefab>,
    paused_ui: Handle<UiPrefab>,
}

impl<'a, 'b> DemoState {
    pub fn new(
        prefabs: Prefabs,
        fps_ui: Handle<UiPrefab>,
        paused_ui: Handle<UiPrefab>,
    ) -> DemoState {
        DemoState {
            prefabs,
            fps_ui,
            paused_ui,
        }
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
        let discrete_pos = DiscretePos::default();
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            (discrete_pos.x * 50 + 50) as f32,
            (discrete_pos.x * 50 + 50) as f32,
            0.0,
        );
        let scale_factor = 100.0 / 32.0;
        transform.set_scale(Vector3::new(scale_factor, scale_factor, 1.0));
        //Create player:
        world
            .create_entity()
            .with(self.prefabs.get_mob())
            .with(transform)
            .with(discrete_pos)
            .with(Velocity::default())
            .with(Steering::new(discrete_pos))
            .with(PlayerTag)
            .build();

        let mut ghost_transform = Transform::default();
        ghost_transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
        world
            .create_entity()
            .with(self.prefabs.get_frame())
            .with(ghost_transform)
            .with(DebugSteeringGhostTag)
            .build();
        world
            .create_entity()
            .with(self.prefabs.get_frame())
            .with(Transform::default())
            .with(DebugPosGhostTag)
            .build();
        initialise_camera(world);
        setup_debug_lines(world);
        world.write_resource::<History>().force_key_frame = true;
        initialize_sprite(world, self.prefabs.get_background());
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
        .with(CameraFrameTag)
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
