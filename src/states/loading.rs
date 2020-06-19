use crate::game_data::CustomGameData;
use crate::states::DemoState;
use amethyst::audio::output::init_output;
use amethyst::core::math::Vector3;
use amethyst::prelude::WorldExt;
use amethyst::renderer::rendy::texture::image::ImageTextureConfig;
use amethyst::ui::UiCreator;
use amethyst::ui::UiLoader;
use amethyst::ui::UiPrefab;
use amethyst::utils::fps_counter::FpsCounterBundle;
use amethyst::EmptyState;
use amethyst::EmptyTrans;
use amethyst::State;
use amethyst::StateEvent;
use amethyst::{
    animation::{
        get_animation_set, AnimationBundle, AnimationCommand, AnimationControlSet, AnimationSet,
        AnimationSetPrefab, EndControl,
    },
    assets::{
        AssetStorage, Completion, Handle, Loader, Prefab, PrefabData, PrefabLoader,
        PrefabLoaderSystem, Progress, ProgressCounter, RonFormat,
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
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::{prefab::SpriteScenePrefab, SpriteRender},
        types::DefaultBackend,
        Camera, ImageFormat, RenderingBundle, SpriteSheet, SpriteSheetFormat, Texture,
    },
    ui::{Anchor, RenderUi, TtfFormat, UiBundle, UiText, UiTransform},
    utils::application_root_dir,
    window::ScreenDimensions,
    Application, GameData, GameDataBuilder, LogLevelFilter, LoggerConfig, SimpleState, SimpleTrans,
    StateData, StdoutLog, Trans,
};
use log::info;
use precompile::AnimationId;
use precompile::MyPrefabData;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct LoadingState {
    progress: ProgressCounter,
    load_ui: Option<Entity>,
    fps_ui: Option<Handle<UiPrefab>>,
    paused_ui: Option<Handle<UiPrefab>>,
    // mob_prefab: Option<Handle<Prefab<MyPrefabData>>>,
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for LoadingState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        init_output(data.world);
        self.load_ui = Some(data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", &mut self.progress);
            creator.create("ui/loading.ron", &mut self.progress)
        }));
        self.fps_ui = Some(
            data.world
                .exec(|loader: UiLoader<'_>| loader.load("ui/fps.ron", &mut self.progress)),
        );
        self.paused_ui = Some(
            data.world
                .exec(|loader: UiLoader<'_>| loader.load("ui/paused.ron", &mut self.progress)),
        );
        // self.mob_prefab = Some(data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        //     loader.load("prefab/sprite_animation.ron", RonFormat, &mut self.progress)
        // }));
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);
        match self.progress.complete() {
            Completion::Failed => {
                eprintln!("Failed loading assets");
                Trans::Quit
            }
            Completion::Complete => {
                println!("Assets loaded, swapping state");
                if let Some(entity) = self.load_ui {
                    let _ = data.world.delete_entity(entity);
                }
                Trans::Switch(Box::new(DemoState::new(
                    // self.mob_prefab.as_ref().unwrap().clone(),
                    self.fps_ui.as_ref().unwrap().clone(),
                    self.paused_ui.as_ref().unwrap().clone(),
                )))
            }
            Completion::Loading => Trans::None,
        }
    }
}
