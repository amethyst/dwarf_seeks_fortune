use crate::components::Player;
use crate::components::Velocity;
use crate::game_data::CustomGameData;
use crate::states::PausedState;
use amethyst::core::math::Vector3;
use amethyst::prelude::WorldExt;
use amethyst::ui::UiPrefab;
use amethyst::State;
use amethyst::StateEvent;
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    assets::{Handle, Prefab},
    core::transform::Transform,
    ecs::{prelude::World, Entities, Join, ReadStorage, WriteStorage},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::Builder,
    renderer::{sprite::SpriteRender, Camera},
    window::ScreenDimensions,
    StateData, Trans,
};
use log::info;
use precompile::AnimationId;
use precompile::MyPrefabData;

// #[derive(Default)]
pub struct DemoState {
    mob_prefab: Handle<Prefab<MyPrefabData>>,
    fps_ui: Handle<UiPrefab>,
    paused_ui: Handle<UiPrefab>,
}
impl DemoState {
    pub fn new(
        mob_prefab: Handle<Prefab<MyPrefabData>>,
        fps_ui: Handle<UiPrefab>,
        paused_ui: Handle<UiPrefab>,
    ) -> DemoState {
        DemoState {
            mob_prefab,
            fps_ui,
            paused_ui,
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for DemoState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        let StateData { world, .. } = data;
        let mut transform = Transform::default();
        transform.set_scale(Vector3::new(10.0, 10.0, 10.0));
        world
            .create_entity()
            .with(self.mob_prefab.clone())
            .with(transform)
            .with(Player {
                velocity: Velocity { x: 5.0, y: 5.0 },
            })
            .build();
        initialise_camera(world);
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        let StateData { world, .. } = data;
        // Execute a pass similar to a system
        world.exec(
            |(entities, animation_sets, mut control_sets): (
                Entities,
                ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
                WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
            )| {
                // For each entity that has AnimationSet
                for (entity, animation_set) in (&entities, &animation_sets).join() {
                    // Creates a new AnimationControlSet for the entity
                    let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                    // Adds the `Fly` animation to AnimationControlSet and loops infinitely
                    control_set.add_animation(
                        AnimationId::Fly,
                        &animation_set.get(&AnimationId::Fly).unwrap(),
                        EndControl::Loop(None),
                        1.0,
                        AnimationCommand::Start,
                    );
                }
            },
        );
        data.data.update(&world, true);
        Trans::None
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::F1) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Escape) {
                // Pause the game by going to the `PausedState`.
                Trans::Push(Box::new(PausedState::new(
                    data.world
                        .create_entity()
                        .with(self.paused_ui.clone())
                        .build(),
                )))
            } else {
                // Listen to any key events
                if let Some(event) = get_key(&event) {
                    info!("handling key event: {:?}", event);
                }
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.fetch::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}
