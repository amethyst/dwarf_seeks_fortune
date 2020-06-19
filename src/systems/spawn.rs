use crate::components::Health;
use crate::components::Velocity;
use precompile::MyPrefabData;
use amethyst::renderer::SpriteSheet;

use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::{
        prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
        Entities,
    },
    assets::{Handle, Prefab},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
    window::ScreenDimensions,
};

use rand::prelude::*;

const GLOB_MAX_SPEED: f32 = 5.0;
const GLOB_ACCELERATION: [f32; 2] = [0.5, 1.0];
const GLOB_FRICTION: f32 = 0.9;
const GLOB_GRAVITY: f32 = -0.5;

#[derive(Default)]
pub struct SpawnSystem {
    cooldown: f32,
}

impl SpawnSystem {
    pub fn new() -> Self {
        SpawnSystem { cooldown: 1.0 }
    }
}

impl<'s> System<'s> for SpawnSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Health>,
        WriteStorage<'s, Handle<Prefab<MyPrefabData>>>,
        Entities<'s>,
        ReadExpect<'s, Handle<Prefab<MyPrefabData>>>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            mut velocities,
            mut healths,
            mut spriterenders,
            entities,
            mob_prefab,
            input,
            time,
            screen_dimens,
        ): Self::SystemData,
    ) {
        let delta = time.delta_seconds();
        self.cooldown -= delta;
        if self.cooldown <= 0.0 {
            self.cooldown = 1.0;
        } else {
            return;
        }
        // let mut rng = rand::thread_rng();
        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, screen_dimens.height() * 0.5, 0.0);
        let velocity = Velocity::new(1.0, 0.0);
        entities
            .build_entity()
            .with(transform.clone(), &mut transforms)
            .with(mob_prefab.clone(), &mut spriterenders)
            .with(velocity, &mut velocities)
            .with(Health::new(100), &mut healths)
            .build();
    }
}
