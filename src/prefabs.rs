use amethyst::{
    assets::{Completion, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    renderer::{Texture, SpriteSheet},
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    StateData, Trans,
};
use precompile::MyPrefabData;

#[derive(Clone, Debug)]
pub struct Prefabs {
    // bg_texture: Handle<Texture>,
    bg_spritesheet: Handle<SpriteSheet>,

    mob: Handle<Prefab<MyPrefabData>>,
    frame: Handle<Prefab<MyPrefabData>>,
}

impl Prefabs {
    pub fn new(bg_spritesheet: Handle<SpriteSheet>, mob: Handle<Prefab<MyPrefabData>>, frame: Handle<Prefab<MyPrefabData>>) -> Prefabs {
        Prefabs {
            bg_spritesheet,
            mob,
            frame,
        }
    }
    // pub fn get_background(&self) -> Handle<Texture> {
    //     self.bg_texture.clone()
    // }
    pub fn get_background(&self) -> Handle<SpriteSheet> {
        self.bg_spritesheet.clone()
    }
    pub fn get_mob(&self) -> Handle<Prefab<MyPrefabData>> {
        self.mob.clone()
    }
    pub fn get_frame(&self) -> Handle<Prefab<MyPrefabData>> {
        self.frame.clone()
    }
}