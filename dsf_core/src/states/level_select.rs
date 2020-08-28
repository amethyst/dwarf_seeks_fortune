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
use crate::states::{window_event_handler, PlayState};
use crate::systems;
use crate::utility::files::{get_adventures_dir, get_levels_dir};
use amethyst::core::ecs::{Dispatcher, DispatcherBuilder, Read};
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

    /// Call this when the user tries to select a node.
    /// This function will check what node the user currently has selected and act accordingly.
    ///
    /// - If the user selected a road, nothing will happen.
    /// - If the user selected a level, that level will be opened in the Play state.
    /// - If the user selected an adventure, that adventure will be opened in a nested LevelSelect state.
    fn select_node(world: &mut World) -> SimpleTrans {
        world.exec(
            |(adventure, pos_on_map): (Read<Adventure>, Read<PositionOnMap>)| {
                let selected_node = adventure.nodes.get(&pos_on_map.pos);
                match selected_node {
                    Some(MapElement::Node(AdventureNode {
                        details: NodeDetails::Level(level_name),
                        ..
                    })) => {
                        let play_state = PlayState::new(get_levels_dir().join(level_name));
                        Trans::Push(Box::new(play_state))
                    }
                    _ => Trans::None,
                }
            },
        )
    }

    fn perform_setup(&self, world: &mut World) {
        create_camera(world);
        load_adventure(&self.adventure_file, world).expect("Failed to load adventure!");
    }
}

impl SimpleState for LevelSelectState {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("LevelSelectState on_start");
        self.dispatcher.setup(data.world);
        self.perform_setup(data.world);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        info!("LevelSelectState on_stop");
        data.world.delete_all();
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("LevelSelectState on_pause");
        data.world.delete_all();
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("LevelSelectState on_resume");
        self.perform_setup(data.world);
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
            StateEvent::Input(input_event) => match input_event {
                InputEvent::KeyReleased {
                    key_code: VirtualKeyCode::Return,
                    scancode: _,
                } => Self::select_node(data.world),
                _ => Trans::None,
            },
        }
    }

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.dispatcher.dispatch(&data.world);
        Trans::None
    }
}
