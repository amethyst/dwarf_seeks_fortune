use crate::levels::Map;
use amethyst::{
    prelude::{Config, World, WorldExt},
    utils::application_root_dir,
};

pub fn save(world: &mut World) {
    let level_file = application_root_dir()
        .expect("Root dir not found!")
        .join("assets/")
        .join("tiles/")
        .join("generated.ron");
    let mut map = world.write_resource::<Map>();
    (&*map).write(level_file);
}
