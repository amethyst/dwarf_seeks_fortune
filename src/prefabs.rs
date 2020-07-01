use crate::game_data::CustomGameData;
use crate::states::DemoState;
use amethyst::audio::output::init_output;
use amethyst::prelude::WorldExt;
use amethyst::ui::UiCreator;
use amethyst::ui::UiLoader;
use amethyst::ui::UiPrefab;
use amethyst::State;
use amethyst::StateEvent;
use amethyst::{
    assets::{Completion, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    StateData, Trans,
};
use precompile::MyPrefabData;

#[derive(Clone, Debug)]
pub struct Prefabs {
    mob: Handle<Prefab<MyPrefabData>>,
    frame: Handle<Prefab<MyPrefabData>>,
}

impl Prefabs {
    pub fn new(mob: Handle<Prefab<MyPrefabData>>, frame: Handle<Prefab<MyPrefabData>>) -> Prefabs {
        Prefabs {
            mob,
            frame,
        }
    }

    pub fn get_mob(&self) -> Handle<Prefab<MyPrefabData>> {
        self.mob.clone()
    }
    pub fn get_frame(&self) -> Handle<Prefab<MyPrefabData>> {
        self.frame.clone()
    }
}