use amethyst::{
    ecs::{prelude::World, Entities, Join, ReadStorage},
    utils::application_root_dir,
};

use crate::components::*;
use dsf_core::levels::load_level;

pub fn setup_test(test: MovementTest, world: &mut World) {
    clear_previous_test(world);
    load_level_from_file(&test, world);
}

fn clear_previous_test(world: &mut World) {
    world.exec(
        |(entities, tags): (Entities, ReadStorage<MovementTestScopeTag>)| {
            for (entity, _) in (&entities, &tags).join() {
                entities
                    .delete(entity)
                    .expect("Failed to clear entities belonging to the previous test.");
            }
        },
    );
}

fn load_level_from_file(test: &MovementTest, world: &mut World) {
    let file_name = match test {
        MovementTest::Jump2Wide => "jump_2_wide.ron",
        MovementTest::Jump4Wide => "jump_4_wide.ron",
    };
    let level_file = application_root_dir()
        .expect("Root dir not found!")
        .join("../assets/")
        .join("tests/")
        .join(file_name);
    load_level(&level_file, world).expect("Failed to load level!");
}
