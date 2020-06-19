use crate::game_data::CustomGameData;
use amethyst::audio::output::init_output;
use amethyst::core::math::Vector3;
use amethyst::prelude::WorldExt;
use amethyst::renderer::rendy::texture::image::ImageTextureConfig;
use amethyst::ui::UiCreator;
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
        AssetStorage, Handle, Loader, PrefabData, PrefabLoader, PrefabLoaderSystem, Progress,
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
