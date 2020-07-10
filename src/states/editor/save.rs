use crate::levels::Level;
use crate::resources::EditorData;
use amethyst::{
    prelude::{Config, World, WorldExt},
    utils::application_root_dir,
};

pub fn save(world: &mut World) {
    let level_file = application_root_dir()
        .expect("Root dir not found!")
        .join("assets/")
        .join("levels/")
        .join("generated.ron");
    let mut data = world.write_resource::<EditorData>();
    let level: Level = (&*data).level.clone().into();
    level.write(level_file);
}
