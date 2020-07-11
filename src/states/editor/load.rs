use amethyst::{
    config::ConfigError,
    prelude::{Config, World, WorldExt},
    utils::application_root_dir,
};

use crate::levels::Level;
use crate::resources::{EditorData, LevelEdit};

pub fn load(world: &mut World) -> Result<LevelEdit, ConfigError> {
    let level_file = application_root_dir()
        .expect("Root dir not found!")
        .join("assets/")
        .join("levels/")
        .join("generated.ron");
    let level = Level::load(level_file)?;
    Ok(level.into())
}
