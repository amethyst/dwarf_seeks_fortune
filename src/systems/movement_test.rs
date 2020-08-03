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
pub struct TestSetupSystem;

impl<'s> System<'s> for TestSetupSystem {
    type SystemData = (Entities<'s>, ReadStorage<'s, MovementTestScopeTag>);

    fn run(&mut self, (entities, tags): Self::SystemData) {}
}
