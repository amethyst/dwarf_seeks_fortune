#![allow(dead_code, unused_imports, unused_variables)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate log;

mod states;

use amethyst::prelude::{Config, SystemExt};
use amethyst::{
    assets::{PrefabLoaderSystemDesc, Processor},
    audio::Source,
    core::SystemDesc,
    prelude::*,
    utils::application_root_dir,
    Application, GameDataBuilder, LoggerConfig,
};
use dsf_checks::systems as checks_systems;

use dsf_core::resources::Tile::Dummy;
use dsf_core::resources::{CurrentState, DebugConfig, MovementConfig};
use dsf_core::systems;
use dsf_editor::resources::EditorConfig;
use dsf_editor::systems as editor_systems;
use dsf_precompile::PrecompiledDefaultsBundle;
use dsf_precompile::PrecompiledRenderBundle;
use dsf_precompile::{start_game, MyPrefabData};

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(LoggerConfig::default()).start();
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("../assets/");
    let config_dir = assets_dir.join("config/");
    let display_config_path = config_dir.join("display.ron");
    let bindings_config_path = config_dir.join("input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(PrecompiledDefaultsBundle {
            bindings_config_path,
        })?
        .with_system_desc(
            PrefabLoaderSystemDesc::<MyPrefabData>::default(),
            "prefab_loader",
            &[],
        )
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with(
            systems::FpsCounterUiSystem::default(),
            "fps_counter_ui_system",
            &[],
        )
        .with(systems::CameraSystem, "camera_system", &[])
        .with(
            systems::CameraControlSystem,
            "camera_control_system",
            &["camera_system"],
        )
        .with(systems::DummySystem, "dummy_system", &[])
        .with_bundle(PrecompiledRenderBundle {
            display_config_path,
        })?;

    start_game(
        assets_dir,
        game_data,
        Some(Box::new(states::LoadingState::default())),
    );
    Ok(())
}
