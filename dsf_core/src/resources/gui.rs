use std::collections::HashMap;

use amethyst::prelude::WorldExt;

use amethyst::ui::UiPrefab;

use amethyst::{assets::Handle, ecs::prelude::Entity, prelude::*};

#[derive(Default, Debug)]
pub struct UiHandles {
    map: HashMap<UiType, Handle<UiPrefab>>,
}

impl UiHandles {
    pub fn put_handle(mut self, key: UiType, handle: Handle<UiPrefab>) -> Self {
        self.map.insert(key, handle);
        self
    }

    pub fn clone_handle(&self, key: &UiType) -> Handle<UiPrefab> {
        (*self
            .map
            .get(key)
            .unwrap_or_else(|| panic!("Tried loading UI element {:?} but failed!", key)))
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
    /// A message in the center of the screen that tells you that you completed the level.
    WinMessage,
    /// Dialog that pops up when you want to save a level in the editor.
    Save,
    /// Ui for the level editor.
    Editor,
    /// The paused menu. Not currently in use, but will be implemented in the future.
    Paused,
    /// The main menu.
    MainMenu,
}
