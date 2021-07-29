use amethyst::{
    Application, CoreApplication, GameData, GameDataBuilder, SimpleState, SimpleTrans, StateData,
    StateEvent, Trans,
};
use std::path::PathBuf;

/// A wrapper for the real state that we want to start the game with.
struct MainState {
    real_state: Option<Box<dyn SimpleState>>,
}

/// This wrapper-implementation simply delegates all calls to the inner state.
impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(ref mut state) = self.real_state {
            state.on_start(data);
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(ref mut state) = self.real_state {
            state.on_stop(data);
        }
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(ref mut state) = self.real_state {
            state.on_pause(data);
        }
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(ref mut state) = self.real_state {
            state.on_resume(data);
        }
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        self.real_state
            .as_mut()
            .map_or(Trans::None, |state| state.handle_event(data, event))
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.real_state
            .as_mut()
            .map_or(Trans::None, |state| state.update(data))
    }
}

// saves ~2 seconds
pub fn start_game(
    resources: PathBuf,
    game_data_builder: GameDataBuilder<'static, 'static>,
    state: Option<Box<dyn SimpleState>>,
) {
    let mut game: Application<'_, GameData<'_, '_>> =
        CoreApplication::build(resources, MainState { real_state: state })
            .unwrap()
            // .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
            .build(game_data_builder)
            .unwrap();
    game.run();
}
