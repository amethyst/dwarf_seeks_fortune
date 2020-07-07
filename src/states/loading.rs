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
use crate::states::{DemoState, EditorState};

#[derive(Default)]
pub struct LoadingState {
    progress: ProgressCounter,
    load_ui: Option<Entity>,
    fps_ui: Option<Handle<UiPrefab>>,
    paused_ui: Option<Handle<UiPrefab>>,
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

        let mut assets = Assets::default();
        assets.put_still(
            SpriteType::NotFound,
            load_spritesheet(
                "textures/not_found.png",
                "prefab/still_not_found.ron",
                data.world,
                &mut self.progress,
            ),
        );
        // assets.put_still(
        //     SpriteType::Background,
        //     load_spritesheet(
        //         "textures/background.jpg",
        //         "prefab/still_background.ron",
        //         data.world,
        //         &mut self.progress,
        //     ),
        // );
        assets.put_still(
            SpriteType::Frame,
            load_spritesheet(
                "textures/frame.png",
                "prefab/still_frame.ron",
                data.world,
                &mut self.progress,
            ),
        );
        assets.put_still(
            SpriteType::Blocks,
            load_spritesheet(
                "textures/blocks.png",
                "prefab/still_blocks.ron",
                data.world,
                &mut self.progress,
            ),
        );
        assets.put_still(
            SpriteType::Selection,
            load_spritesheet(
                "textures/selection.png",
                "prefab/still_selection.ron",
                data.world,
                &mut self.progress,
            ),
        );
        assets.put_animated(
            AnimType::NotFound,
            load_animation("prefab/anim_not_found.ron", data.world, &mut self.progress),
        );
        assets.put_animated(
            AnimType::Mob,
            load_animation("prefab/anim_mob.ron", data.world, &mut self.progress),
        );
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
        let editor_mode = (&*data.world.read_resource::<DebugConfig>()).editor_mode;
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
                if editor_mode {
                    Trans::Switch(Box::new(EditorState::new(
                        self.fps_ui.as_ref().unwrap().clone(),
                    )))
                } else {
                    Trans::Switch(Box::new(DemoState::new(
                        self.fps_ui.as_ref().unwrap().clone(),
                        self.paused_ui.as_ref().unwrap().clone(),
                    )))
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
