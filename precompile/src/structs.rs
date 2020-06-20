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
    prelude::World,
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

impl Component for MyPrefabData {
    type Storage = DenseVecStorage<Self>;
}
