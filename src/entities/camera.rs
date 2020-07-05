use crate::components::*;
use amethyst::{
    core::{transform::Transform, Parent},
    prelude::{Builder, World, WorldExt},
    renderer::{
        formats::texture::ImageFormat, palette::Srgba, resources::Tint, sprite::SpriteRender,
        Camera, SpriteSheet, Texture,
    },
    window::ScreenDimensions,
};

/// Initialise the camera.
pub fn initialise_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.fetch::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    let camera_frame = world
        .create_entity()
        .with(CameraFrameTag::default())
        .with(transform)
        .build();

    world
        .create_entity()
        .with(Parent {
            entity: camera_frame,
        })
        .with(Camera::standard_2d(width, height))
        .with(Transform::default())
        .build();
}
