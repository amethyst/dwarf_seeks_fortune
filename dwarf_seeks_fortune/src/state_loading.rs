use amethyst::prelude::WorldExt;
use amethyst::ui::UiCreator;
use amethyst::ui::UiLoader;
use dsf_core::resources::{Assets, DebugConfig, MovementConfig, Music, UiHandles};

use amethyst::{
    assets::{AssetStorage, Completion, Loader, PrefabLoader, ProgressCounter, RonFormat},
    ecs::prelude::Entity,
    prelude::*,
    renderer::{formats::texture::ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    StateData, Trans,
};
use dsf_precompile::MyPrefabData;

use crate::loading_config::LoadingConfig;
use crate::state_main_menu::MainMenuState;
use amethyst::audio::{AudioSink, Mp3Format, WavFormat};
use amethyst::utils::application_root_dir;
use dsf_editor::resources::EditorConfig;

/// This state is briefly active when the game is first started up. It loads all assets used in the
/// entire game and then switches to the main menu state.
///
/// If you want to add a new asset that should be loaded, please go to LoadingConfig and add it
/// there.
#[derive(Default)]
pub struct LoadingState {
    progress: ProgressCounter,
    load_ui: Option<Entity>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        load_configs(data.world);
        self.load_ui = Some(data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/loading.ron", &mut self.progress)
        }));

        // Create a LoadingConfig to tell us what assets to actually load.
        let mut loading_config = load_loading_config();

        // Load all UI handles.
        let ui_handles =
            loading_config
                .uis
                .drain(..)
                .fold(UiHandles::default(), |handles, (ui_type, path)| {
                    handles.put_handle(
                        ui_type,
                        data.world
                            .exec(|loader: UiLoader<'_>| loader.load(path, &mut self.progress)),
                    )
                });
        data.world.insert(ui_handles);

        // Load all sprite sheets for still images and add them to an Assets instance.
        let assets = loading_config.stills.drain(..).fold(
            Assets::default(),
            |assets, (sprite_type, texture_path, spritesheet_path)| {
                let loader = data.world.read_resource::<Loader>();
                let texture_handle = loader.load(
                    texture_path,
                    ImageFormat::default(),
                    &mut self.progress,
                    &data.world.read_resource::<AssetStorage<Texture>>(),
                );
                assets.put_still(
                    sprite_type,
                    loader.load(
                        spritesheet_path,
                        SpriteSheetFormat(texture_handle),
                        &mut self.progress,
                        &data.world.read_resource::<AssetStorage<SpriteSheet>>(),
                    ),
                )
            },
        );
        // Take the Assets instance we previously filled with still images and add animations.
        let assets =
            loading_config
                .animations
                .drain(..)
                .fold(assets, |assets, (anim_type, prefab_path)| {
                    assets.put_animated(
                        anim_type,
                        data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
                            loader.load(prefab_path, RonFormat, &mut self.progress)
                        }),
                    )
                });
        // Take the Assets instance we previously filled with stills and animations and
        // add sound effects.
        let assets = loading_config.sound_effects.drain(..).fold(
            assets,
            |assets, (sound_type, file_path)| {
                let loader = data.world.read_resource::<Loader>();
                assets.put_sound(
                    sound_type,
                    loader.load(
                        file_path,
                        WavFormat,
                        &mut self.progress,
                        &data.world.read_resource(),
                    ),
                )
            },
        );
        data.world.insert(assets);

        // Set audio volume:
        data.world.write_resource::<AudioSink>().set_volume(0.25);
        // Create a music player.
        let music_handles = loading_config
            .music_tracks
            .drain(..)
            .map(|music_file_path| {
                let loader = data.world.read_resource::<Loader>();
                loader.load(
                    music_file_path,
                    Mp3Format,
                    &mut self.progress,
                    &data.world.read_resource(),
                )
            })
            .collect();
        data.world.insert(Music::new(music_handles));
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        match self.progress.complete() {
            Completion::Failed => {
                error!("Failed loading assets");
                Trans::Quit
            }
            Completion::Complete => {
                info!("Assets loaded, swapping state");
                if let Some(entity) = self.load_ui {
                    let _ = data.world.delete_entity(entity);
                }
                Trans::Switch(Box::new(MainMenuState::new()))
            }
            Completion::Loading => Trans::None,
        }
    }
}

fn load_loading_config() -> LoadingConfig {
    let config_dir = application_root_dir()
        .expect("Failed to get application root directory!")
        .join("../assets/")
        .join("config/");
    LoadingConfig::load(&config_dir.join("loading.ron")).unwrap_or_else(|error| {
        error!(
            "Failed to load loading config! Falling back to default. Error: {:?}",
            error
        );
        LoadingConfig::default()
    })
}

fn load_configs(world: &mut World) {
    let config_dir = application_root_dir()
        .expect("Failed to get application root directory!")
        .join("../assets/")
        .join("config/");
    world.insert(
        DebugConfig::load(&config_dir.join("debug.ron")).unwrap_or_else(|error| {
            error!(
                "Failed to load debug config! Falling back to default. Error: {:?}",
                error
            );
            DebugConfig::default()
        }),
    );
    world.insert(
        MovementConfig::load(&config_dir.join("movement.ron")).unwrap_or_else(|error| {
            error!(
                "Failed to load movement config! Falling back to default. Error: {:?}",
                error
            );
            MovementConfig::default()
        }),
    );
    world.insert(
        EditorConfig::load(&config_dir.join("editor.ron")).unwrap_or_else(|error| {
            error!(
                "Failed to load editor config! Falling back to default. Error: {:?}",
                error
            );
            EditorConfig::default()
        }),
    );
}
