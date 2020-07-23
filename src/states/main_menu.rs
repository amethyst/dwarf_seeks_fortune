use crate::game_data::CustomGameData;
use crate::resources::*;
use crate::states::*;
use amethyst::prelude::{Builder, World};
use amethyst::{
    ecs::prelude::{Entity, WorldExt},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    ui::{UiEvent, UiEventType, UiFinder},
    State, StateData, StateEvent, Trans,
};

const PLAY_BUTTON_ID: &str = "play";
const EDITOR_BUTTON_ID: &str = "editor";
const EXIT_BUTTON_ID: &str = "exit";

#[derive(Default)]
pub struct MainMenuState {
    ui: Option<Entity>,
    play_button: Option<Entity>,
    editor_button: Option<Entity>,
    exit_button: Option<Entity>,
}

impl MainMenuState {
    pub fn new() -> MainMenuState {
        MainMenuState::default()
    }

    fn init_ui(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        UiHandles::add_ui(&UiType::Fps, data.world);
        self.ui = UiHandles::add_ui(&UiType::MainMenu, data.world);
        // invoke a world update to finish creating our ui entities
        data.data.update(&data.world, false);
        // look up our buttons
        data.world.exec(|ui_finder: UiFinder<'_>| {
            self.play_button = ui_finder.find(PLAY_BUTTON_ID);
            self.editor_button = ui_finder.find(EDITOR_BUTTON_ID);
            self.exit_button = ui_finder.find(EXIT_BUTTON_ID);
        });
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        info!("MainMenuState on_start");
        self.init_ui(data);
    }

    fn on_pause(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        info!("MainMenuState on_pause");
        data.world.delete_all();
        self.play_button = None;
        self.editor_button = None;
        self.exit_button = None;
    }

    fn on_resume(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        info!("MainMenuState on_resume");
        self.init_ui(data);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.play_button {
                    Trans::Push(Box::new(PlayState::demo()))
                } else if Some(target) == self.editor_button {
                    Trans::Push(Box::new(EditorState))
                } else if Some(target) == self.exit_button {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false);
        Trans::None
    }
}
