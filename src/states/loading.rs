use amethyst::{
    assets::{AssetStorage, Completion, Handle, Loader, PrefabLoader, ProgressCounter, RonFormat},
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{formats::texture::ImageFormat, SpriteSheet, SpriteSheetFormat, Texture}, StateData,
    Trans,
};
use amethyst::audio::output::init_output;
use amethyst::prelude::WorldExt;
use amethyst::State;
use amethyst::StateEvent;
use amethyst::ui::UiCreator;
use amethyst::ui::UiLoader;
use amethyst::ui::UiPrefab;
use precompile::MyPrefabData;

use crate::game_data::CustomGameData;
use crate::prefabs::Prefabs;
use crate::states::DemoState;

#[derive(Default)]
pub struct LoadingState {
    progress: ProgressCounter,
    load_ui: Option<Entity>,
    fps_ui: Option<Handle<UiPrefab>>,
    paused_ui: Option<Handle<UiPrefab>>,
    prefabs: Option<Prefabs>,
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for LoadingState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        init_output(data.world);
        self.load_ui = Some(data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", &mut self.progress);
            creator.create("ui/loading.ron", &mut self.progress)
        }));
        self.fps_ui = Some(
            data.world
                .exec(|loader: UiLoader<'_>| loader.load("ui/fps.ron", &mut self.progress)),
        );
        self.paused_ui = Some(
            data.world
                .exec(|loader: UiLoader<'_>| loader.load("ui/paused.ron", &mut self.progress)),
        );
        let mob_prefab = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/sprite_animation.ron", RonFormat, &mut self.progress)
        });
        let frame_prefab = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/frame_animation.ron", RonFormat, &mut self.progress)
        });
        let background_handle = load_texture("sprites/background.jpg", &data.world, &mut self.progress);
        let bg_spritesheet = load_spritesheet("sprites/background.ron", background_handle, &data.world, &mut self.progress);
        data.world.insert(mob_prefab.clone());
        data.world.insert(frame_prefab.clone());
        data.world.insert(bg_spritesheet.clone());
        self.prefabs = Some(Prefabs::new(bg_spritesheet, mob_prefab, frame_prefab));
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);
        match self.progress.complete() {
            Completion::Failed => {
                eprintln!("Failed loading assets");
                Trans::Quit
            }
            Completion::Complete => {
                println!("Assets loaded, swapping state");
                if let Some(entity) = self.load_ui {
                    let _ = data.world.delete_entity(entity);
                }
                Trans::Switch(Box::new(DemoState::new(
                    self.prefabs.as_ref().unwrap().clone(),
                    self.fps_ui.as_ref().unwrap().clone(),
                    self.paused_ui.as_ref().unwrap().clone(),
                )))
            }
            Completion::Loading => Trans::None,
        }
    }
}

pub fn load_texture<N>(name: N, world: &World, progress: &mut ProgressCounter) -> Handle<Texture> where N: Into<String>, {
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        ImageFormat::default(),
        progress,
        &world.read_resource::<AssetStorage<Texture>>(),
    )
}

pub fn load_spritesheet<N>(name: N, texture_handle: Handle<Texture>, world: &World, progress: &mut ProgressCounter) -> Handle<SpriteSheet> where N: Into<String>, {
    let loader = world.read_resource::<Loader>();
    loader.load(name, SpriteSheetFormat(texture_handle), progress, &world.read_resource::<AssetStorage<SpriteSheet>>())
}