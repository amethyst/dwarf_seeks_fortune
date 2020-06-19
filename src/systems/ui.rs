use amethyst::{
    core::{
        math::{UnitQuaternion, Vector3},
        Time, Transform,
    },
    ecs::{
        prelude::{Entity, Join, Read, ReadStorage, System, WriteExpect, WriteStorage},
        ReaderId, Write,
    },
    renderer::{camera::Camera, light::Light},
    shrev::EventChannel,
    ui::{UiEvent, UiFinder, UiText},
    utils::fps_counter::FpsCounter,
};
use log::info;

#[derive(Default)]
pub struct UiSystem {
    fps_display: Option<Entity>,
}

impl<'a> System<'a> for UiSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, UiText>,
        Read<'a, FpsCounter>,
        UiFinder<'a>,
    );

    fn run(&mut self, (time, mut ui_text, fps_counter, finder): Self::SystemData) {
        if self.fps_display.is_none() {
            if let Some(fps_entity) = finder.find("fps_text") {
                self.fps_display = Some(fps_entity);
            }
        }
        if let Some(fps_entity) = self.fps_display {
            if let Some(fps_display) = ui_text.get_mut(fps_entity) {
                if time.frame_number() % 20 == 0 {
                    let fps = fps_counter.sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }
    }
}

/// This shows how to handle UI events.
#[derive(Default)]
pub struct UiEventHandlerSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl UiEventHandlerSystem {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> System<'a> for UiEventHandlerSystem {
    type SystemData = Write<'a, EventChannel<UiEvent>>;

    fn run(&mut self, mut events: Self::SystemData) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| events.register_reader());

        // Reader id was just initialized above if empty
        for ev in events.read(reader_id) {
            info!("[SYSTEM] You just interacted with a ui element: {:?}", ev);
        }
    }
}
