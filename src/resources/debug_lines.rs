//! Displays debug lines using an orthographic camera.

use amethyst::{
    core::{
        transform::{Transform, TransformBundle},
        Parent, Time,
    },
    derive::SystemDesc,
    ecs::{prelude::*, Entity, Read, ReadExpect, System, SystemData, WorldExt, Write},
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
    // world.insert(DebugLines::new());
    // Configure width of lines. Optional step
    // world.insert(DebugLinesParams { line_width: 2.0 });

    // Setup debug lines as a component and add lines to render axis&grid
    let mut debug_lines_component = DebugLinesComponent::new();

    for y in (0..(100 as u16)).step_by(2).skip(1).map(f32::from) {
        debug_lines_component.add_line(
            [-100., y, 0.0].into(),
            [100., y, 0.0].into(),
            Srgba::new(0.3, 0.3, 0.3, 0.5),
        );
        debug_lines_component.add_line(
            [-100., -y, 0.0].into(),
            [100., -y, 0.0].into(),
            Srgba::new(0.3, 0.3, 0.3, 0.5),
        );
    }

    for x in (0..(100 as u16)).step_by(2).skip(1).map(f32::from) {
        debug_lines_component.add_line(
            [x, -100., 0.0].into(),
            [x, 100., 0.0].into(),
            Srgba::new(0.3, 0.3, 0.3, 0.5),
        );
        debug_lines_component.add_line(
            [-x, -100., 0.0].into(),
            [-x, 100., 0.0].into(),
            Srgba::new(0.3, 0.3, 0.3, 0.5),
        );
    }

    debug_lines_component.add_line(
        [-5000., 0.0, 0.0].into(),
        [5000., 0.0, 0.0].into(),
        Srgba::new(1.0, 0.0, 0.0, 0.8),
    );
    debug_lines_component.add_line(
        [0.0, -5000., 0.0].into(),
        [0.0, 5000., 0.0].into(),
        Srgba::new(1.0, 0.0, 0.0, 0.8),
    );

    world.create_entity().with(debug_lines_component).build();
}
