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
use precompile::MyPrefabData;

use crate::game_data::CustomGameData;
use crate::resources::*;
use crate::states::{EditorState, MainMenuState};

#[derive(Default)]
pub struct LoadingState {
    progress: ProgressCounter,
    load_ui: Option<Entity>,
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for LoadingState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        init_output(data.world);
        self.load_ui = Some(data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/loading.ron", &mut self.progress)
        }));
        let ui_handles = vec![
            (UiType::Fps, "ui/fps.ron"),
            (UiType::Paused, "ui/paused.ron"),
            (UiType::MainMenu, "ui/main_menu.ron"),
        ]
        .drain(..)
        .fold(UiHandles::default(), |mut handles, (ui_type, path)| {
            handles.put_handle(
                ui_type,
                data.world
                    .exec(|loader: UiLoader<'_>| loader.load(path, &mut self.progress)),
            )
        });
        data.world.insert(ui_handles);

        let mut assets = vec![
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
                SpriteType::Selection,
                "textures/selection.png",
                "prefab/still_selection.ron",
            ),
        ]
        .drain(..)
        .fold(
            Assets::default(),
            |mut assets, (sprite_type, texture_path, ron_path)| {
                assets.put_still(
                    sprite_type,
                    load_spritesheet(texture_path, ron_path, data.world, &mut self.progress),
                )
            },
        );
        let assets = vec![
            (AnimType::NotFound, "prefab/anim_not_found.ron"),
            (AnimType::Mob, "prefab/anim_mob.ron"),
        ]
        .drain(..)
        .fold(assets, |mut assets, (anim_type, prefab_path)| {
            assets.put_animated(
                anim_type,
                load_animation(prefab_path, data.world, &mut self.progress),
            )
        });
        data.world.insert(assets);
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
        let skip_straight_to_editor =
            (&*data.world.read_resource::<DebugConfig>()).skip_straight_to_editor;
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
                if skip_straight_to_editor {
                    Trans::Switch(Box::new(EditorState))
                } else {
                    Trans::Switch(Box::new(MainMenuState::new()))
                }
            }
            Completion::Loading => Trans::None,
        }
    }
}

pub fn load_texture<N>(name: N, world: &World, progress: &mut ProgressCounter) -> Handle<Texture>
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

pub fn load_spritesheet<N>(
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

pub fn load_animation<N>(
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
