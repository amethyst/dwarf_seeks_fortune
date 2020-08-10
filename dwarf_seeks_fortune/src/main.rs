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
    Application, LoggerConfig,
};
use dsf_checks::systems as checks_systems;
use dsf_core::game_data::CustomGameDataBuilder;
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
    let game_data = CustomGameDataBuilder::default()
        .with_base_bundle(
            &mut app_builder.world,
            PrecompiledDefaultsBundle {
                bindings_config_path: &bindings_config_path,
            },
        )?
        .with_core(
            PrefabLoaderSystemDesc::<MyPrefabData>::default().build(&mut app_builder.world),
            "scene_loader",
            &[],
        )
        .with_core(Processor::<Source>::new(), "source_processor", &[])
        .with_core(
            systems::UiEventHandlerSystem::new(),
            "ui_event_handler",
            &[],
        )
        .with_core(
            systems::FpsCounterUiSystem::default(),
            "fps_counter_ui_system",
            &[],
        )
        .with_core(
            systems::PlayerSystem::default().pausable(CurrentState::Running),
            "player_system",
            &["input_system"],
        )
        .with_core(
            systems::SteeringSystem::default().pausable(CurrentState::Running),
            "steering_system",
            &["player_system"],
        )
        .with_core(
            systems::MovementSystem.pausable(CurrentState::Running),
            "movement_system",
            &["steering_system"],
        )
        .with_core(
            systems::VelocitySystem.pausable(CurrentState::Running),
            "velocity_system",
            &["movement_system"],
        )
        .with_core(systems::DebugSystem, "debug_system", &["input_system"])
        .with_core(systems::CameraSystem, "camera_system", &[])
        .with_core(
            systems::CameraControlSystem,
            "camera_control_system",
            &["camera_system"],
        )
        .with_core(
            systems::RewindControlSystem,
            "rewind_control_system",
            &["player_system"],
        )
        .with_core(
            systems::RewindSystem.pausable(CurrentState::Rewinding),
            "rewind_system",
            &["rewind_control_system", "input_system"],
        )
        .with_core(
            editor_systems::CursorPreviewSystem,
            "cursor_preview_system",
            &[],
        )
        .with_core(editor_systems::CursorSystem, "cursor_system", &[])
        .with_core(
            editor_systems::SelectionSystem,
            "selection_system",
            &["cursor_system"],
        )
        .with_core(
            editor_systems::TilePaintSystem,
            "tile_paint_system",
            &["selection_system"],
        )
        .with_core(
            checks_systems::TestSetupSystem::default(),
            "test_setup_system",
            &["input_system"],
        )
        .with_core(systems::KeyCollectionSystem, "key_collection_system", &[])
        .with_core(systems::PickupSystem, "pickup_system", &[])
        .with_core(systems::UseToolSystem, "use_tool_system", &[])
        .with_core(systems::LevelWrappingSystem, "level_wrapping_system", &[])
        .with_core(systems::WinSystem, "win_system", &[])
        .with_base_bundle(
            &mut app_builder.world,
            PrecompiledRenderBundle {
                display_config_path: &display_config_path,
            },
        )?;
    let mut game = app_builder.build(game_data)?;
    game.run();
    Ok(())
}
