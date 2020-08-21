use dsf_core::resources::{AnimType, SoundType, SpriteType, UiType};
use serde::{Deserialize, Serialize};

/// This specifies all assets that must be loaded by the LoadingState.
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct LoadingConfig {
    pub uis: Vec<(UiType, String)>,
    pub animations: Vec<(AnimType, String)>,
    pub stills: Vec<(SpriteType, String, String)>,
    pub sound_effects: Vec<(SoundType, String)>,
    pub music_tracks: Vec<String>,
}

impl Default for LoadingConfig {
    fn default() -> Self {
        LoadingConfig {
            uis: vec![],
            animations: vec![(AnimType::NotFound, "prefab/anim_not_found.ron".to_string())],
            stills: vec![(
                SpriteType::NotFound,
                "textures/not_found.png".to_string(),
                "prefab/still_not_found.ron".to_string(),
            )],
            sound_effects: vec![],
            music_tracks: vec![],
        }
    }
}

// impl LoadingConfig {
//     fn music_to_be_loaded() -> Vec<String> {
//         let music_dir = application_root_dir()
//             .expect("Root dir not found!")
//             .join("../assets/")
//             .join("audio/")
//             .join("music/");
//         fs::read_dir(music_dir)
//             .expect("Failed to read contents of the assets/audio/msuic/ directory.")
//             .map(|file| {
//                 if let Ok(file) = file {
//                     if file.path().is_file() {
//                         Some(
//                             "../assets/audio/music/".to_string()
//                                 + file
//                                     .path()
//                                     .file_name()
//                                     .expect("This should not happen.")
//                                     .to_str()
//                                     .expect("Music file name did not contain valid unicode."),
//                         )
//                     } else {
//                         None
//                     }
//                 } else {
//                     None
//                 }
//             })
//             .filter(|option| option.is_some())
//             .map(|option| option.unwrap())
//             .collect()
//     }
// }
