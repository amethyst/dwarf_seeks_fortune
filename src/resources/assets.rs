use serde::{Deserialize, Serialize};
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
            .or_else(|| {
                println!("Spritesheet asset {:?} is missing!", asset_type);
                self.stills.get(&SpriteType::NotFound)
            })
            .expect(&format!("Fallback asset also missing.")))
            .clone()
    }

    pub fn get_animated(&self, asset_type: AnimType) -> Handle<Prefab<MyPrefabData>> {
        (*self
            .animated
            .get(&asset_type)
            .or_else(|| {
                println!("Animation asset {:?} is missing!", asset_type);
                self.animated.get(&AnimType::NotFound)
            })
            .expect(&format!("Fallback asset also missing!")))
            .clone()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AssetType {
    Still(SpriteType),
    Animated(AnimType),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum SpriteType {
    /// This is the fallback sprite to use if the desired sprite cannot be found.
    NotFound,
    Background,
    Frame,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum AnimType {
    /// The fallback animated asset to use if the desired asset could not be found.
    NotFound,
    Mob,
}
