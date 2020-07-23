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

/// Sets up a grid of grey debug lines, spaced 2 meters apart.
/// The x and y axes are drawn in red to make them stand out.
pub fn setup_debug_lines(world: &mut World) {
    let mut debug_lines_component = DebugLinesComponent::new();

    // Adds all the horizontal lines, spaced 2 meters apart.
    // No line is drawn at y=0, the x-axis will be added later in a different colour.
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

    // Adds all the vertical lines, spaced 2 meters apart.
    // No line is drawn at x=0, the y-axis will be added later in a different colour.
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

    // Adds the x-axis and the y-axis as red lines.
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
