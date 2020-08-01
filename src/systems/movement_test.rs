use crate::components::*;
use crate::levels::*;
use crate::resources::*;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Entities, ReaderId};
use amethyst::input::{InputEvent, VirtualKeyCode};
use amethyst::ui::UiEvent;
use amethyst::utils::application_root_dir;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

/// This shows how to handle UI events.
#[derive(Default)]
pub struct TestSetupSystem {
    reader_id: Option<ReaderId<InputEvent<StringBindings>>>,
}

impl<'s> System<'s> for TestSetupSystem {
    type SystemData = (
        Write<'s, EventChannel<InputEvent<StringBindings>>>,
        Entities<'s>,
        ReadStorage<'s, MovementTestScopeTag>,
    );

    fn run(&mut self, (mut events, entities, tags): Self::SystemData) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| events.register_reader());

        // Reader id was just initialized above if empty
        for event in events.read(reader_id) {
            match event {
                InputEvent::KeyPressed {
                    key_code: VirtualKeyCode::Key1,
                    ..
                } => {
                    // clear_previous_test(&entities, &tags);
                    // let level_file = application_root_dir()
                    //     .expect("Root dir not found!")
                    //     .join("assets/")
                    //     .join("tests/")
                    //     .join("jump_2_wide.ron");
                    // load_level(&level_file, data.world);
                }
                InputEvent::KeyPressed {
                    key_code: VirtualKeyCode::Key2,
                    ..
                } => {
                    clear_previous_test(&entities, &tags);
                }
                InputEvent::KeyPressed {
                    key_code: VirtualKeyCode::Key3,
                    ..
                } => {}
                _ => (),
            }
        }
    }
}

fn clear_previous_test(entities: &Entities, tags: &ReadStorage<MovementTestScopeTag>) {
    for (entity, _) in (entities, tags).join() {
        entities.delete(entity);
    }
}

fn load(test: MovementTest) {

}