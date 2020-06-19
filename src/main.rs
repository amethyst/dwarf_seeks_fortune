#![allow(
    dead_code,
    unused_must_use,
    unused_imports,
    unused_variables,
    unused_parens,
    unused_mut
)]

extern crate amethyst;
extern crate derive_new;
extern crate log;
extern crate rand;
extern crate serde;
extern crate specs_derive;

mod components;
mod game_data;
mod states;
mod systems;

use game_data::CustomGameDataBuilder;
use precompile::MyPrefabData;
use precompile::PrecompiledDefaultsBundle;
use precompile::PrecompiledRenderBundle;

use amethyst::audio::Source;
use amethyst::core::SystemDesc;
use amethyst::ecs::ReaderId;
use amethyst::ecs::System;
use amethyst::ecs::Write;
use amethyst::shrev::EventChannel;
use amethyst::ui::UiEvent;

use amethyst::prelude::WorldExt;
use amethyst::utils::fps_counter::FpsCounterBundle;

use amethyst::{
    animation::{
        get_animation_set, AnimationBundle, AnimationCommand, AnimationControlSet, AnimationSet,
        AnimationSetPrefab, EndControl,
    },
    assets::{
        PrefabData, PrefabLoader, PrefabLoaderSystem, PrefabLoaderSystemDesc, Processor,
        ProgressCounter, RonFormat,
    },
    core::transform::{Transform, TransformBundle},
    derive::PrefabData,
    ecs::{prelude::Entity, Entities, Join, ReadStorage, WriteStorage},
    error::Error,
    input::{
        get_key, is_close_requested, is_key_down, InputBundle, StringBindings, VirtualKeyCode,
    },
    prelude::{Builder, World},
    renderer::{
        camera::Camera,
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::{prefab::SpriteScenePrefab, SpriteRender},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    window::ScreenDimensions,
    Application, GameData, GameDataBuilder, LogLevelFilter, LoggerConfig, SimpleState, SimpleTrans,
    StateData, StdoutLog, Trans,
};
use log::info;
use serde::{Deserialize, Serialize};

fn main() {
    let result = make_game();
    if let Err(e) = result {
        println!("Error starting game: {:?}", e);
    }
}

fn make_game() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default()).start();
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets/");
    let config_dir = assets_dir.join("config/");
    let display_config_path = config_dir.join("display.ron");
    let bindings_config_path = config_dir.join("bindings.ron");

    let mut app_builder = Application::build(assets_dir, states::LoadingState::default())?;
    let mut world = &mut app_builder.world;

    let game_data = CustomGameDataBuilder::default()
        .with_base_bundle(
            world,
            PrecompiledDefaultsBundle {
                bindings_config_path: &bindings_config_path,
            },
        )?
        .with_core(
            PrefabLoaderSystemDesc::<MyPrefabData>::default().build(world),
            "scene_loader",
            &[],
        )
        .with_core(Processor::<Source>::new(), "source_processor", &[])
        .with_core(
            systems::UiEventHandlerSystem::new(),
            "ui_event_handler",
            &[],
        )
        .with_core(systems::UiSystem::default(), "ui_system", &[])
        .with_core(
            systems::MovementSystem,
            "movement_system",
            &["input_system"],
        )
        .with_base_bundle(
            world,
            PrecompiledRenderBundle {
                display_config_path: &display_config_path,
            },
        )?;
    let mut game = app_builder.build(game_data)?;
    game.run();
    Ok(())
}
