use amethyst::utils::application_root_dir;
use dsf_core::resources::{AnimType, SoundType, SpriteType, UiType};
use std::fs;

/// This specifies all assets that must be loaded by the LoadingState.
pub struct LoadingConfig {
    pub uis: Vec<(UiType, &'static str)>,
    pub animations: Vec<(AnimType, &'static str)>,
    pub stills: Vec<(SpriteType, &'static str, &'static str)>,
    pub sound_effects: Vec<(SoundType, &'static str)>,
    pub music_tracks: Vec<String>,
}

impl LoadingConfig {
    pub fn new() -> Self {
        LoadingConfig {
            uis: Self::uis_to_be_loaded(),
            animations: Self::animations_to_be_loaded(),
            stills: Self::stills_to_be_loaded(),
            music_tracks: Self::music_to_be_loaded(),
            sound_effects: Self::sound_effects_to_be_loaded(),
        }
    }

    fn uis_to_be_loaded() -> Vec<(UiType, &'static str)> {
        vec![
            (UiType::Fps, "ui/fps.ron"),
            (UiType::WinMessage, "ui/win_message.ron"),
            (UiType::Save, "ui/save.ron"),
            (UiType::Editor, "ui/editor.ron"),
            (UiType::MainMenu, "ui/main_menu.ron"),
        ]
    }

    fn animations_to_be_loaded() -> Vec<(AnimType, &'static str)> {
        vec![
            (AnimType::NotFound, "prefab/anim_not_found.ron"),
            (AnimType::Mob, "prefab/anim_mob.ron"),
            (AnimType::Miner, "prefab/anim_miner.ron"),
        ]
    }

    fn stills_to_be_loaded() -> Vec<(SpriteType, &'static str, &'static str)> {
        vec![
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
    }

    fn music_to_be_loaded() -> Vec<String> {
        let music_dir = application_root_dir()
            .expect("Root dir not found!")
            .join("../assets/")
            .join("audio/")
            .join("music/");
        fs::read_dir(music_dir)
            .expect("Failed to read contents of the assets/audio/msuic/ directory.")
            .map(|file| {
                if let Ok(file) = file {
                    if file.path().is_file() {
                        Some(
                            "../assets/audio/music/".to_string()
                                + file
                                    .path()
                                    .file_name()
                                    .expect("This should not happen.")
                                    .to_str()
                                    .expect("Music file name did not contain valid unicode."),
                        )
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .filter(|option| option.is_some())
            .map(|option| option.unwrap())
            .collect()
    }

    fn sound_effects_to_be_loaded() -> Vec<(SoundType, &'static str)> {
        vec![
            (SoundType::Jump, "audio/jump01.wav"),
            (SoundType::Step, "audio/step01.wav"),
            (SoundType::Step, "audio/step02.wav"),
            (SoundType::Step, "audio/step03.wav"),
            (SoundType::Step, "audio/step04.wav"),
            (SoundType::Step, "audio/step05.wav"),
            (SoundType::Step, "audio/step06.wav"),
            (SoundType::Step, "audio/step07.wav"),
            (SoundType::CannotPerformAction, "audio/nonono.wav"),
        ]
    }
}
