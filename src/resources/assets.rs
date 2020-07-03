use amethyst::{
    assets::{Completion, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    renderer::{SpriteSheet, Texture},
    StateData, Trans,
};
use precompile::MyPrefabData;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Assets {
    stills: HashMap<SpriteType, Handle<SpriteSheet>>,
    animated: HashMap<AnimType, Handle<Prefab<MyPrefabData>>>,
}

impl Assets {
    pub fn put_still(&mut self, asset_type: SpriteType, asset: Handle<SpriteSheet>) {
        self.stills.insert(asset_type, asset);
    }

    pub fn put_animated(&mut self, asset_type: AnimType, asset: Handle<Prefab<MyPrefabData>>) {
        self.animated.insert(asset_type, asset);
    }

    pub fn get_still(&self, asset_type: SpriteType) -> Handle<SpriteSheet> {
        (*self
            .stills
            .get(&asset_type)
            .expect(&format!("Spritesheet asset missing: {:?}", asset_type)))
        .clone()
    }

    pub fn get_animated(&self, asset_type: AnimType) -> Handle<Prefab<MyPrefabData>> {
        (*self
            .animated
            .get(&asset_type)
            .expect(&format!("Animated asset missing: {:?}", asset_type)))
        .clone()
    }
}

// TODO: Remove this??
#[derive(Debug)]
pub enum AssetType {
    Still(SpriteType),
    Animated(AnimType),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SpriteType {
    Background,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AnimType {
    Mob,
    Frame,
}
