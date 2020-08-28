use amethyst::utils::application_root_dir;
use std::fs;
use std::path::PathBuf;

pub fn get_adventures_dir() -> PathBuf {
    create_if_missing(get_world_dir().join("adventures/"))
}

pub fn get_levels_dir() -> PathBuf {
    create_if_missing(get_world_dir().join("levels/"))
}

fn get_world_dir() -> PathBuf {
    application_root_dir()
        .expect("Root dir not found!")
        .join("../assets/")
        .join("world/")
}

fn create_if_missing(path: PathBuf) -> PathBuf {
    fs::create_dir_all(&path).expect(&format!("Failed to create directory {:?}", &path));
    path
}
