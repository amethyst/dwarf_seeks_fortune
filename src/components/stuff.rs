use amethyst::ecs::VecStorage;
use amethyst::core::Named;
use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;
use amethyst::utils::fps_counter::FpsCounterBundle;
use amethyst::{
    LogLevelFilter, LoggerConfig, StdoutLog,
    animation::{
        get_animation_set, AnimationBundle, AnimationCommand, AnimationControlSet, AnimationSet,
        AnimationSetPrefab, EndControl,
    },
    input::{InputBundle, StringBindings, get_key, is_close_requested, is_key_down, VirtualKeyCode},
    assets::{PrefabData, PrefabLoader, PrefabLoaderSystem, ProgressCounter, RonFormat},
    core::transform::{Transform, TransformBundle},
    derive::PrefabData,
    ecs::{prelude::Entity, Entities, Join, ReadStorage, WriteStorage},
    error::Error,
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
    Application, GameData, GameDataBuilder, SimpleState, SimpleTrans, StateData, Trans,
};
use serde::{Deserialize, Serialize};
use log::info;

pub struct Ship {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Ship {
    pub fn new(velocity: [f32; 2], radius: f32) -> Ship {
        Ship {
            velocity: velocity,
            radius: radius,
        }
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Velocity{
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Deserialize, Serialize, PrefabData)]
pub struct Player {
    pub velocity: Velocity,
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}
