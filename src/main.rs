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

use amethyst::audio::Source;
use amethyst::ecs::ReaderId;
use amethyst::ecs::System;
use amethyst::ecs::Write;
use amethyst::shrev::EventChannel;
use amethyst::ui::UiEvent;
use states::DemoState;
use states::LoadingState;

use amethyst::prelude::WorldExt;
use amethyst::utils::fps_counter::FpsCounterBundle;
use amethyst::{
    animation::{
        get_animation_set, AnimationBundle, AnimationCommand, AnimationControlSet, AnimationSet,
        AnimationSetPrefab, EndControl,
    },
    assets::{PrefabData, PrefabLoader, PrefabLoaderSystem, Processor, ProgressCounter, RonFormat},
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

/// Animation ids used in a AnimationSet
#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
    Fly,
}

/// Loading data for one entity
#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct MyPrefabData {
    /// Information for rendering a scene with sprites
    sprite_scene: SpriteScenePrefab,
    /// Аll animations that can be run on the entity
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}

fn main() {
    make_game();
}

fn make_game() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        // .level_for("gfx_device_gl", amethyst::LogLevelFilter::Warn)
        // .level_for("gfx_glyph", amethyst::LogLevelFilter::Error)
        .start();

    // amethyst::start_logger(LoggerConfig {
    //     stdout: StdoutLog::Colored,
    //     level_filter: LogLevelFilter::Info,
    //     log_file: None,
    //     allow_env_override: true,
    //     log_gfx_rendy_level: Some(LogLevelFilter::Warn),
    // });
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("assets/config/display.ron");

    let assets_dir = app_root.join("assets/");
    let mut app_builder = Application::build(assets_dir, LoadingState::default())?;
    let mut world = &mut app_builder.world;

    // let mut world = World::new();
    let game_data = CustomGameDataBuilder::default()
        // .with_base(
        //     PrefabLoaderSystem::<MyPrefabData>::default(),
        //     "scene_loader",
        //     &[],
        // )
        .with_base_bundle(
            &mut world,
            AnimationBundle::<AnimationId, SpriteRender>::new(
                "sprite_animation_control",
                "sprite_sampler_interpolation",
            ),
        )?
        .with_base_bundle(
            &mut world,
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with_base_bundle(&mut world, FpsCounterBundle {})?
        .with_base_bundle(
            &mut world,
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(app_root.join("src/config/bindings.ron"))?,
        )?
        .with_base_bundle(&mut world, UiBundle::<StringBindings>::new())?
        .with_core(Processor::<Source>::new(), "source_processor", &[])
        .with_core(UiEventHandlerSystem::new(), "ui_event_handler", &[])
        .with_core(systems::UiSystem::default(), "ui_system", &[])
        // The renderer must be executed on the same thread consecutively, so we initialize it as thread_local
        // which will always execute on the main thread.
        .with_base_bundle(
            &mut world,
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    // let mut game = Application::new(assets_dir, LoadingState::default(), game_data)?;
    let mut game = app_builder.build(game_data)?;
    game.run();
    Ok(())
}

/// This shows how to handle UI events.
#[derive(Default)]
pub struct UiEventHandlerSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl UiEventHandlerSystem {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> System<'a> for UiEventHandlerSystem {
    type SystemData = Write<'a, EventChannel<UiEvent>>;

    fn run(&mut self, mut events: Self::SystemData) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| events.register_reader());

        // Reader id was just initialized above if empty
        for ev in events.read(reader_id) {
            info!("[SYSTEM] You just interacted with a ui element: {:?}", ev);
        }
    }
}
