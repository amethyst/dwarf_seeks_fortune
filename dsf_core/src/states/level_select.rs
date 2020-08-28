use std::path::PathBuf;

use amethyst::prelude::WorldExt;

use amethyst::StateEvent;
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    core::timing::Time,
    ecs::{prelude::World, Entities, Join, ReadStorage, WriteStorage},
    input::{is_close_requested, is_key_down, InputEvent, VirtualKeyCode},
    renderer::sprite::SpriteRender,
    utils::application_root_dir,
    StateData, Trans,
};
use amethyst::{GameData, SimpleState, SimpleTrans};

use dsf_precompile::AnimationId;

use crate::entities::*;
use crate::levels::*;
use crate::resources::*;
use crate::states::window_event_handler;
use crate::systems;
use crate::utility::files::get_adventures_dir;
use amethyst::core::ecs::{Dispatcher, DispatcherBuilder};
use amethyst::core::SystemExt;

/// This can be used to either select an adventure from the world or a level from an adventure.
pub struct LevelSelectState {
    dispatcher: Dispatcher<'static, 'static>,
    adventure_file: PathBuf,
}

impl<'a, 'b> LevelSelectState {
    /// Creates a LevelSelectState that starts in demo mode. It loads the default adventure.
    pub fn demo() -> Self {
        let adventure_file = get_adventures_dir().join("default.ron");
        LevelSelectState::new(adventure_file)
    }

    /// Creates a new LevelSelectState that will load the given adventure.
    pub fn new(adventure_file: PathBuf) -> Self {
        LevelSelectState {
            adventure_file,
            dispatcher: DispatcherBuilder::new()
                .with(systems::MapCursorSystem, "map_cursor_system", &[])
                .build(),
        }
    }

    fn handle_action(&mut self, action: &str, world: &mut World) -> SimpleTrans {
        Trans::None
    }
}

impl SimpleState for LevelSelectState {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("LevelSelectState on_start");
        self.dispatcher.setup(data.world);
        create_camera(data.world);
        load_adventure(&self.adventure_file, data.world).expect("Failed to load adventure!");
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        info!("LevelSelectState on_stop");
        data.world.delete_all();
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        window_event_handler::handle(&event, data.world);
        match event {
            // Events related to the window and inputs.
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Pop
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

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.dispatcher.dispatch(&data.world);
        Trans::None
    }
}
