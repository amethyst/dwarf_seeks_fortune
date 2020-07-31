use std::path::{Path, PathBuf};

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
    core::{timing::Time, transform::Transform, Parent},
    ecs::{prelude::World, Entities, Entity, Join, ReadStorage, WriteStorage},
    input::{get_key, is_close_requested, is_key_down, InputEvent, VirtualKeyCode},
    prelude::*,
    renderer::{
        formats::texture::ImageFormat, palette::Srgba, resources::Tint, sprite::SpriteRender,
        Camera, SpriteSheet, Texture,
    },
    utils::application_root_dir,
    window::{MonitorIdent, ScreenDimensions, Window},
    winit::{Event, WindowEvent},
    StateData, Trans,
};

use precompile::AnimationId;

use crate::components::*;
use crate::entities::*;
use crate::game_data::CustomGameData;
use crate::levels::*;
use crate::resources::*;
use crate::states::{window_event_handler, EditorState, PausedState};

pub struct PlayState {
    level_file: PathBuf,
    editor_mode: bool,
}

impl<'a, 'b> PlayState {
    /// Creates a PlayState that starts in demo mode. It loads the demo level.
    pub fn demo() -> Self {
        let level_file = application_root_dir()
            .expect("Root dir not found!")
            .join("assets/")
            .join("levels/")
            .join("demo_level.ron");
        PlayState {
            level_file,
            editor_mode: false,
        }
    }

    /// Creates a new PlayState that will load the given level.
    pub fn new(level_file: PathBuf, editor_mode: bool) -> Self {
        PlayState {
            level_file,
            editor_mode,
        }
    }

    fn handle_action(
        &mut self,
        action: &str,
        world: &mut World,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if action == "speedUp" {
            let (old_scale, new_scale) = (*world.fetch_mut::<DebugConfig>()).increase_speed();
            info!("Speeding up time, from x{:?} to x{:?}. This feature exists for debugging purposes only.", old_scale, new_scale);
            self.update_time_scale(world, new_scale);
            Trans::None
        } else if action == "slowDown" {
            let (old_scale, new_scale) = (*world.fetch_mut::<DebugConfig>()).decrease_speed();
            info!("Slowing down time, from x{:?} to x{:?}. This feature exists for debugging purposes only.", old_scale, new_scale);
            self.update_time_scale(world, new_scale);
            Trans::None
        } else {
            Trans::None
        }
    }

    fn update_time_scale(&self, world: &mut World, time_scale: f32) {
        world.write_resource::<Time>().set_time_scale(time_scale);
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for PlayState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        info!("PlayState on_start");
        let StateData { world, .. } = data;
        UiHandles::add_ui(&UiType::Fps, world);
        create_camera(world);
        world.insert(History::default());
        load_level(&self.level_file, world);
    }

    fn on_stop(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        info!("PlayState on_stop");
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
                if is_close_requested(&event) {
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    if self.editor_mode {
                        Trans::Switch(Box::new(EditorState))
                    } else {
                        Trans::Pop
                    }
                } else {
                    Trans::None
                }
            }
            // Ui event. Button presses, mouse hover, etc...
            StateEvent::Ui(_) => Trans::None,
            StateEvent::Input(input_event) => {
                if let InputEvent::ActionPressed(action) = input_event {
                    self.handle_action(&action, data.world)
                } else {
                    Trans::None
                }
            }
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
