use crate::components::*;
use amethyst::{
    core::{math::Vector3, transform::Transform, Parent},
    ecs::Entity,
    prelude::{Builder, World, WorldExt},
    renderer::{
        formats::texture::ImageFormat, palette::Srgba, resources::Tint, sprite::SpriteRender,
        Camera, SpriteSheet, Texture,
    },
    window::ScreenDimensions,
};

/// Initialise the camera.
pub fn create_camera(world: &mut World, root: &Entity) {
    let frame = initialise_camera_frame(world, root);
    create_camera_under_parent(world, frame);
}

pub fn create_camera_under_parent(world: &mut World, parent: Entity) {
    let (width, height) = {
        let dim = world.fetch::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    world
        .create_entity()
        .with(Parent { entity: parent })
        .with(Camera::standard_2d(width, height))
        .with(Transform::default())
        .build();
}

pub fn initialise_camera_frame(world: &mut World, root: &Entity) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);
    transform.set_scale(Vector3::new(1. / 50., 1. / 50., 1.0));
    world
        .create_entity()
        .with(Parent {
            entity: root.clone(),
        })
        .with(CameraFrame::default())
        .with(transform)
        .build()
}
