use crate::game_data::CustomGameData;
use amethyst::{
    ecs::prelude::{Entity, WorldExt},
    input::{is_key_down, VirtualKeyCode},
    State, StateData, StateEvent, Trans,
};

pub struct PausedState {
    ui: Entity,
}

impl PausedState {
    pub fn new(ui: Entity) -> PausedState {
        PausedState { ui }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for PausedState {
    // fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
    //     let StateData { world, .. } = data;
    //     init_output(&mut world.res);
    //
    //     println!("PausedState on_start");
    // }

    fn handle_event(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                let _ = data.world.delete_entity(self.ui);
                // Go back to the `GameplayState`.
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
