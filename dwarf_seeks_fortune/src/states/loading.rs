use amethyst::audio::output::init_output;
use amethyst::prelude::WorldExt;
use amethyst::ui::UiCreator;
use amethyst::ui::UiLoader;
use dsf_core::resources::{
    AnimType, Assets, DebugConfig, MovementConfig, SpriteType, UiHandles, UiType,
};
use dsf_core::states::window_event_handler;

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
use dsf_precompile::MyPrefabData;

use crate::states::MainMenuState;
use amethyst::utils::application_root_dir;
use dsf_editor::resources::EditorConfig;

#[derive(Default)]
pub struct LoadingState {
    progress: ProgressCounter,
    load_ui: Option<Entity>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        load_configs(data.world);
        init_output(data.world);
        self.load_ui = Some(data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/loading.ron", &mut self.progress)
        }));
        let ui_handles = vec![
            (UiType::Fps, "ui/fps.ron"),
            (UiType::WinMessage, "ui/win_message.ron"),
            (UiType::Save, "ui/save.ron"),
            (UiType::Editor, "ui/editor.ron"),
            (UiType::Paused, "ui/paused.ron"),
            (UiType::MainMenu, "ui/main_menu.ron"),
        ]
        .drain(..)
        .fold(UiHandles::default(), |handles, (ui_type, path)| {
            handles.put_handle(
                ui_type,
                data.world
                    .exec(|loader: UiLoader<'_>| loader.load(path, &mut self.progress)),
            )
        });
        data.world.insert(ui_handles);

        let assets = vec![
            (
                SpriteType::NotFound,
                "textures/not_found.png",
                "prefab/still_not_found.ron",
            ),
            (
                SpriteType::Ladder,
                "textures/ladder.png",
                "prefab/still_ladder.ron",
            ),
            (
                SpriteType::Frame,
                "textures/frame.png",
                "prefab/still_frame.ron",
            ),
            (
                SpriteType::Blocks,
                "textures/blocks.png",
                "prefab/still_blocks.ron",
            ),
            (
                SpriteType::Tools,
                "textures/tools.png",
                "prefab/still_tools.ron",
            ),
            (
                SpriteType::Door,
                "textures/door.png",
                "prefab/still_door.ron",
            ),
            (
                SpriteType::Selection,
                "textures/selection.png",
                "prefab/still_selection.ron",
            ),
        ]
        .drain(..)
        .fold(
            Assets::default(),
            |assets, (sprite_type, texture_path, ron_path)| {
                assets.put_still(
                    sprite_type,
                    load_spritesheet(texture_path, ron_path, data.world, &mut self.progress),
                )
            },
        );
        let assets = vec![
            (AnimType::NotFound, "prefab/anim_not_found.ron"),
            (AnimType::Mob, "prefab/anim_mob.ron"),
            (AnimType::Miner, "prefab/anim_miner.ron"),
        ]
        .drain(..)
        .fold(assets, |assets, (anim_type, prefab_path)| {
            assets.put_animated(
                anim_type,
                load_animation(prefab_path, data.world, &mut self.progress),
            )
        });
        data.world.insert(assets);
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        window_event_handler::handle(&event, data.world);
        if let StateEvent::Window(event) = event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
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

fn load_texture<N>(name: N, world: &World, progress: &mut ProgressCounter) -> Handle<Texture>
where
    N: Into<String>,
{
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        ImageFormat::default(),
        progress,
        &world.read_resource::<AssetStorage<Texture>>(),
    )
}

fn load_spritesheet<N>(
    texture_name: N,
    spritesheet_name: N,
    world: &World,
    mut progress: &mut ProgressCounter,
) -> Handle<SpriteSheet>
where
    N: Into<String>,
{
    let texture_handle = load_texture(texture_name, &world, &mut progress);
    let loader = world.read_resource::<Loader>();
    loader.load(
        spritesheet_name,
        SpriteSheetFormat(texture_handle),
        progress,
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}

fn load_animation<N>(
    prefab_name: N,
    world: &mut World,
    progress: &mut ProgressCounter,
) -> Handle<Prefab<MyPrefabData>>
where
    N: Into<String>,
{
    world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        loader.load(prefab_name, RonFormat, progress)
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
