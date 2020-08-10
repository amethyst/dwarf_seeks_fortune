use amethyst::{
    core::Time,
    ecs::{
        prelude::{Entity, Read, System, WriteStorage},
        ReaderId, Write,
    },
    shrev::EventChannel,
    ui::{UiEvent, UiFinder, UiText},
    utils::fps_counter::FpsCounter,
};

#[derive(Default)]
pub struct FpsCounterUiSystem {
    maybe_fps_entity: Option<Entity>,
}

impl<'a> System<'a> for FpsCounterUiSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, UiText>,
        Read<'a, FpsCounter>,
        UiFinder<'a>,
    );

    fn run(&mut self, (time, mut ui_text, fps_counter, finder): Self::SystemData) {
        if time.frame_number() % 20 != 0 {
            // This system should only be executed every 20 frames.
            return;
        }
        // Grab the UiText component from the fps ui entity.
        // If that entity doesn't exist or is expired, try to obtain an up-to-date handle.
        let fps_text = {
            if self.maybe_fps_entity.is_none() {
                self.maybe_fps_entity = finder.find("fps_text");
            }
            if let Some(fps_entity) = self.maybe_fps_entity {
                let maybe_component = ui_text.get_mut(fps_entity);
                if maybe_component.is_none() {
                    self.maybe_fps_entity = finder.find("fps_text");
                }
                maybe_component
            } else {
                None
            }
        };
        if let Some(mut fps_text) = fps_text {
            let fps = fps_counter.sampled_fps();
            fps_text.text = format!("FPS: {:.*}", 2, fps);
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
            debug!("[SYSTEM] You just interacted with a ui element: {:?}", ev);
        }
    }
}
