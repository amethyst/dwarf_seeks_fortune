use crate::components::Pos;
use amethyst::{
    assets::{Completion, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    renderer::{SpriteSheet, Texture},
    StateData, Trans,
};
use precompile::MyPrefabData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Assets {
    stills: HashMap<SpriteType, Handle<SpriteSheet>>,
    animated: HashMap<AnimType, Handle<Prefab<MyPrefabData>>>,
}

impl Assets {
    pub fn put_still(mut self, asset_type: SpriteType, asset: Handle<SpriteSheet>) -> Self {
        self.stills.insert(asset_type, asset);
        self
    }

    pub fn put_animated(
        mut self,
        asset_type: AnimType,
        asset: Handle<Prefab<MyPrefabData>>,
    ) -> Self {
        self.animated.insert(asset_type, asset);
        self
    }

    pub fn get_still(&self, asset_type: &SpriteType) -> Handle<SpriteSheet> {
        (*self
            .stills
            .get(asset_type)
            .or_else(|| {
                error!("Spritesheet asset {:?} is missing!", asset_type);
                self.stills.get(&SpriteType::NotFound)
            })
            .expect(&format!("Fallback asset also missing.")))
        .clone()
    }

    pub fn get_animated(&self, asset_type: &AnimType) -> Handle<Prefab<MyPrefabData>> {
        (*self
            .animated
            .get(asset_type)
            .or_else(|| {
                error!("Animation asset {:?} is missing!", asset_type);
                self.animated.get(&AnimType::NotFound)
            })
            .expect(&format!("Fallback asset also missing!")))
        .clone()
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AssetType {
    /// A static, non-animated image.
    /// Contains both a handle to the sprite sheet and the number of the sprite on the sheet.
    Still(SpriteType, usize),
    /// An animated image.
    Animated(AnimType),
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum SpriteType {
    /// This is the fallback sprite to use if the desired sprite cannot be found.
    NotFound,
    Ladder,
    Background,
    Frame,
    Blocks,
    Selection,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum AnimType {
    /// The fallback animated asset to use if the desired asset could not be found.
    NotFound,
    Mob,
}

/// Matches a still or animated asset to its dimensions in pixels. Required to calculate the
/// correct scale factor for the entity to make it fit within its in-world bounds.
pub fn get_asset_dimensions(asset: &AssetType) -> Pos {
    match asset {
        AssetType::Still(sprite_type, _) => match sprite_type {
            SpriteType::Background => Pos::new(2449, 1632),
            SpriteType::Frame => Pos::new(50, 50),
            SpriteType::Ladder => Pos::new(128, 64),
            _ => Pos::new(128, 128),
        },
        AssetType::Animated(anim_type) => match anim_type {
            AnimType::Mob => Pos::new(32, 32),
            _ => Pos::new(128, 128),
        },
    }
}
