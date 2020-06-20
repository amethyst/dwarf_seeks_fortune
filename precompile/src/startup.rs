use std::path::PathBuf;

use amethyst::{
    animation::{AnimationBundle, AnimationSetPrefab},
    assets::{PrefabData, ProgressCounter},
    core::{transform::TransformBundle, SystemBundle},
    derive::PrefabData,
    ecs::{
        prelude::{Component, Entity},
        DenseVecStorage, DispatcherBuilder, WriteStorage,
    },
    error::Error,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::{prefab::SpriteScenePrefab, SpriteRender},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::fps_counter::FpsCounterBundle,
};
use serde::{Deserialize, Serialize};

// pub struct SplashState<S> {
//     real_state: Option<Box<dyn S>>,
// }
//
// impl SimpleState for SplashState {
//     fn on_start(&mut self, mut data: StateData<GameData>) {
//         if let Some(ref mut state) = self.real_state {
//             state.on_start(data);
//         }
//     }
//
//     fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
//         if let Some(ref mut state) = self.real_state {
//             state.update(data)
//         } else {
//             Trans::None
//         }
//     }
// }
//
// pub fn create_builder(
//     assets_dir: &PathBuf,
//     state: Option<Box<dyn State>>,
// ) -> Result<ApplicationBuilder, Error> {
//     Application::build(assets_dir, SplashState { real_state: state })
// }

// pub fn start_game(
//     builder: ApplicationBuilder<S, T, E, R>,
// ) -> Result<ApplicationBuilder<S, T, E, R>, Error> {
//     Application::build(assets_dir, SplashState { real_state: state })
// }
