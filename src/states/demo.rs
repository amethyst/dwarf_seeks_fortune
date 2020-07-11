use crate::components::*;
use crate::entities::*;
use crate::game_data::CustomGameData;
use crate::levels::*;
use crate::resources::*;
use crate::states::PausedState;
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

pub struct DemoState;

impl<'a, 'b> DemoState {
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
        create_camera(world);
        setup_debug_lines(world);
        world.write_resource::<History>().force_key_frame = true;
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
                    Trans::Push(Box::new(PausedState::new()))
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
