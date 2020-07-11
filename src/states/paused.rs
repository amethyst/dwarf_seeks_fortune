use crate::game_data::CustomGameData;
use crate::resources::*;
use amethyst::prelude::Builder;
use amethyst::{
    ecs::prelude::{Entity, WorldExt},
    input::{is_key_down, VirtualKeyCode},
    State, StateData, StateEvent, Trans,
};

pub struct PausedState {
    ui: Option<Entity>,
}

impl PausedState {
    pub fn new() -> PausedState {
        PausedState { ui: None }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for PausedState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        println!("PausedState on_start");
        self.ui = UiHandles::add_ui(&UiType::Paused, data.world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                if let Some(ui) = self.ui {
                    let _ = data.world.delete_entity(ui);
                }
                return Trans::Pop;
            }
        }

        // Escape isn't pressed, so we stay in this `State`.
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false);
        Trans::None
    }
}
