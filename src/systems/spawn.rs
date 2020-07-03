use crate::components::Health;
use crate::components::Velocity;
use precompile::MyPrefabData;

use amethyst::{
    assets::{Handle, Prefab},
    core::timing::Time,
    core::transform::Transform,
    ecs::{
        prelude::{Read, ReadExpect, System, WriteStorage},
        Entities,
    },
    window::ScreenDimensions,
};

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
        // WriteStorage<'s, Handle<Prefab<MyPrefabData>>>,
        Entities<'s>,
        // ReadExpect<'s, Handle<Prefab<MyPrefabData>>>,
        Read<'s, Time>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            mut velocities,
            mut healths,
            // mut spriterenders,
            entities,
            // mob_prefab,
            time,
            screen_dimens,
        ): Self::SystemData,
    ) {
        // let delta = time.delta_seconds();
        // self.cooldown -= delta;
        // if self.cooldown <= 0.0 && false {
        //     self.cooldown = 1.0;
        // } else {
        //     return;
        // }
        // // let mut rng = rand::thread_rng();
        // let mut transform = Transform::default();
        // transform.set_translation_xyz(0.0, screen_dimens.height() * 0.5, 0.0);
        // let velocity = Velocity::new(60.0, 0.0);
        // entities
        //     .build_entity()
        //     .with(transform, &mut transforms)
        //     .with(mob_prefab.clone(), &mut spriterenders)
        //     .with(velocity, &mut velocities)
        //     .with(Health::new(100), &mut healths)
        //     .build();
    }
}
