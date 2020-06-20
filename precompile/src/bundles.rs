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
use crate::structs::AnimationId;

pub struct PrecompiledRenderBundle<'a> {
    pub display_config_path: &'a PathBuf,
}

impl<'a, 'b> SystemBundle<'a, 'b> for PrecompiledRenderBundle<'_> {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // Saves ~10 seconds.
        // The renderer must be executed on the same thread consecutively, so we initialize it as thread_local
        // which will always execute on the main thread.
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(self.display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with `SpriteRender` component.
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default())
            .build(world, builder)?;
        Ok(())
    }
}

pub struct PrecompiledDefaultsBundle<'a> {
    pub bindings_config_path: &'a PathBuf,
}

impl<'a, 'b, 'c> SystemBundle<'a, 'b> for PrecompiledDefaultsBundle<'c> {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // Saves ~0.5 - 1.5 seconds.
        AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        )
        .build(world, builder)?;

        // saves ~2 seconds
        InputBundle::<StringBindings>::new()
            .with_bindings_from_file(self.bindings_config_path)?
            .build(world, builder)?;

        // Saves ~0.3 seconds
        TransformBundle::new().build(world, builder)?;

        // Saves ~2.25 seconds.
        UiBundle::<StringBindings>::new().build(world, builder)?;
        FpsCounterBundle {}.build(world, builder)?;

        Ok(())
    }
}
