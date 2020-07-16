use std::collections::HashMap;

use amethyst::audio::output::init_output;
use amethyst::prelude::WorldExt;
use amethyst::ui::UiCreator;
use amethyst::ui::UiLoader;
use amethyst::ui::UiPrefab;
use amethyst::State;
use amethyst::StateEvent;
use amethyst::{
    assets::{
        AssetStorage, Completion, Handle, Loader, Prefab, PrefabLoader, ProgressCounter, RonFormat,
    },
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{formats::texture::ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    StateData, Trans,
};

#[derive(Default, Debug)]
pub struct UiHandles {
    map: HashMap<UiType, Handle<UiPrefab>>,
}

impl UiHandles {
    pub fn put_handle(mut self, key: UiType, handle: Handle<UiPrefab>) -> Self {
        self.map.insert(key, handle);
        self
    }

    /// TODO: Get rid of expect call.
    pub fn clone_handle(&self, key: &UiType) -> Handle<UiPrefab> {
        self.map
            .get(key)
            .expect(&format!("Tried loading UI element {:?} but failed!", key))
            .clone()
    }

    /// Convenience method that grabs the correct UiHandle and usues it to create an entity.
    pub fn add_ui(key: &UiType, world: &mut World) -> Option<Entity> {
        let handle = world.read_resource::<UiHandles>().clone_handle(key);
        Some(world.create_entity().with(handle).build())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum UiType {
    /// Small debug FPS meter.
    Fps,
    /// Ui for the level editor.
    Editor,
    /// The paused menu.
    Paused,
    /// The main menu.
    MainMenu,
}
