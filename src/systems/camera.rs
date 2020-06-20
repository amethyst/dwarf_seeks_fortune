use crate::components::*;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{sprite::SpriteRender, Camera},
    window::ScreenDimensions,
};

pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (WriteStorage<'s, Camera>, ReadExpect<'s, ScreenDimensions>);

    fn run(&mut self, (mut cameras, screen_dimens): Self::SystemData) {
        // for (camera) in (&mut cameras).join() {
        //     let (width, height) = (screen_dimens.width(), screen_dimens.height());
        //     let new_camera = Camera::standard_2d(width, height);
        //     camera
        //
        //     let left = -width / 2.0;
        //     let right = width / 2.0;
        //     let bottom = height / 2.0;
        //     let top = -height / 2.0;
        //     camera.set_left_and_right(left, right);
        //     camera.set_bottom_and_top(top, bottom);
        // }
    }
}
