//! Displays debug lines using an orthographic camera.

use amethyst::{
    core::{
        transform::{Transform, TransformBundle},
        Time,
    },
    derive::SystemDesc,
    ecs::{Read, ReadExpect, System, SystemData, WorldExt, Write},
    prelude::*,
    renderer::{
        camera::Camera,
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
        plugins::{RenderDebugLines, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    window::ScreenDimensions,
};

pub fn setup_debug_lines(world: &mut World) {
    // Setup debug lines as a resource
    world.insert(DebugLines::new());
    // Configure width of lines. Optional step
    // world.insert(DebugLinesParams { line_width: 2.0 });

    // Setup debug lines as a component and add lines to render axis&grid
    let mut debug_lines_component = DebugLinesComponent::new();

    let (screen_w, screen_h) = {
        let screen_dimensions = world.read_resource::<ScreenDimensions>();
        (screen_dimensions.width(), screen_dimensions.height())
    };

    for y in (0..(screen_h as u16)).step_by(100).map(f32::from) {
        debug_lines_component.add_line(
            [0.0, y, 0.0].into(),
            [screen_w, y, 0.0].into(),
            Srgba::new(0.3, 0.3, 0.3, 1.0),
        );
    }

    for x in (0..(screen_w as u16)).step_by(100).map(f32::from) {
        debug_lines_component.add_line(
            [x, 0.0, 0.0].into(),
            [x, screen_h, 0.0].into(),
            Srgba::new(0.3, 0.3, 0.3, 1.0),
        );
    }

    world
        .create_entity()
        .with(debug_lines_component)
        .build();
}
