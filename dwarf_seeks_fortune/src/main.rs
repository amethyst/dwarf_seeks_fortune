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
    utils::application_root_dir,
    Application, GameDataBuilder, LoggerConfig,
};
use dsf_checks::systems as checks_systems;

use dsf_core::resources::{CurrentState, DebugConfig, MovementConfig};
use dsf_core::systems;
use dsf_editor::resources::EditorConfig;
use dsf_editor::systems as editor_systems;
use dsf_precompile::MyPrefabData;
use dsf_precompile::PrecompiledDefaultsBundle;
use dsf_precompile::PrecompiledRenderBundle;

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(LoggerConfig::default()).start();
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("../assets/");
    let config_dir = assets_dir.join("config/");
    let display_config_path = config_dir.join("display.ron");
    let bindings_config_path = config_dir.join("input.ron");

    let mut app_builder = Application::build(assets_dir, states::LoadingState::default())?;

    let debug_config = DebugConfig::load(&config_dir.join("debug.ron"))?;
    let editor_config = EditorConfig::load(&config_dir.join("editor.ron"))?;
    let movement_config = MovementConfig::load(&config_dir.join("movement.ron"))?;
    app_builder.world.insert(debug_config);
    app_builder.world.insert(editor_config);
    app_builder.world.insert(movement_config);
    let game_data = GameDataBuilder::default()
        .with_bundle(PrecompiledDefaultsBundle {
            bindings_config_path,
        })?
        .with(
            PrefabLoaderSystemDesc::<MyPrefabData>::default().build(&mut app_builder.world),
            "scene_loader",
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
        .with_bundle(PrecompiledRenderBundle {
            display_config_path,
        })?;
    let mut game = app_builder.build(game_data)?;
    game.run();
    Ok(())
}
