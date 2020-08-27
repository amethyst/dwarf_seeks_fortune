use std::fs;
use std::io::Error;
use std::path::PathBuf;

use amethyst::prelude::*;
use amethyst::utils::application_root_dir;
use serde::{Deserialize, Serialize};

use crate::components::Pos;
use crate::levels::Level;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Adventure {
    nodes: Vec<AdventureNode>,
    roads: Vec<Road>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdventureNode {
    pub id: u16,
    pub name: String,
    pub details: NodeDetails,
    pub pos: Pos,
    /// If true, the player must defeat this node before they can move further.
    /// If false, nodes behind this node are reachable and playable even if this node was never
    /// entered.
    pub blocking: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum NodeDetails {
    /// This node is an adventure: a collection of levels.
    /// Opening this node will push a new LevelSelectState for this adventure.
    Adventure(String),
    /// This node is a level. Opening this node will open the level in the PlayState.
    Level(String),
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Road {
    pub start_id: u16,
    pub end_id: u16,
}

/// Creates a new adventure that gives access to every single level.
/// This is useful while there aren't too many levels yet.
pub fn create_default_adventure() {
    let mut adventure = Adventure::default();
    level_files()
        .iter()
        .map(|level_name| {
            let level_file = get_levels_dir().join(level_name);
            (level_name, Level::load(level_file))
        })
        .filter(|(level_name, result)| {
            result
                .as_ref()
                .or_else(|err| {
                    error!("Failed to load level {:?}: {:?}", level_name, err);
                    Err(err)
                })
                .is_ok()
        })
        .map(|(level_name, result)| (level_name, result.expect("Should never panic.")))
        .enumerate()
        .for_each(|(index, (level_name, level))| {
            adventure.nodes.push(AdventureNode {
                id: index as u16,
                name: level_name.clone(),
                details: NodeDetails::Level(level_name.clone()),
                pos: Pos::new(index as i32 * 2, 0),
                blocking: false,
            });
        });

    adventure
        .write(get_adventures_dir().join("default.ron"))
        .expect("Failed to create default adventure that contains all levels.");
}

fn get_adventures_dir() -> PathBuf {
    get_world_dir().join("adventures/")
}

fn get_levels_dir() -> PathBuf {
    get_world_dir().join("levels/")
}

fn get_world_dir() -> PathBuf {
    application_root_dir()
        .expect("Root dir not found!")
        .join("../assets/")
        .join("world/")
}

fn level_files() -> Vec<String> {
    fs::read_dir(get_levels_dir())
        .expect("Failed to read contents of the levels directory.")
        .map(|file| {
            if let Ok(file) = file {
                if file.path().is_file() {
                    Some(
                        file.path()
                            .file_name()
                            .expect("This should not happen.")
                            .to_str()
                            .expect("Music file name did not contain valid unicode.")
                            .to_string(),
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

pub fn load_adventure(path: &PathBuf, world: &mut World) {
    let adventure = Adventure::load(path);
    println!("{:?}", adventure);
    // for node in adventure.nodes {
    //     world.create_entity().build();
    // }
}
