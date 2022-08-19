use amethyst::{
    ecs::prelude::{Entity, WorldExt},
    input::{is_key_down, VirtualKeyCode},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};

use crate::resources::{UiHandles, UiType};
use crate::states::window_event_handler;

#[derive(Copy, Clone, Default, Debug)]
pub struct PausedState {
    ui: Option<Entity>,
}

impl SimpleState for PausedState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("PausedState on_start");
        self.ui = UiHandles::add_ui(UiType::Paused, data.world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        window_event_handler::handle(&event, data.world);
        if let StateEvent::Window(event) = &event {
            if is_key_down(event, VirtualKeyCode::Escape) {
                if let Some(ui) = self.ui {
                    let _ = data.world.delete_entity(ui);
                }
                return Trans::Pop;
            }
        }

        // Escape isn't pressed, so we stay in this `State`.
        Trans::None
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}
