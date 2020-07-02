use crate::components::*;
use crate::resources::*;
use amethyst::{
    renderer::{palette::Srgba,
               resources::Tint, },
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};
use crate::config::*;
use std::io::SeekFrom::Current;

pub struct RewindControlSystem;

impl<'s> System<'s> for RewindControlSystem {
    type SystemData = (
        Write<'s, CurrentState>,
        Write<'s, Rewind>,
        Write<'s, History>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Read<'s, DebugConfig>,
        WriteStorage<'s, Tint>,
    );

    fn run(&mut self, (mut current_state, mut rewind, mut history, input, time, config, mut tints): Self::SystemData) {
        history.force_key_frame = false;
        if input.action_is_down("rewind").unwrap_or(false) {
            rewind.cooldown = match *current_state {
                CurrentState::Running => config.seconds_per_rewind_frame,
                CurrentState::Rewinding => {
                    if rewind.is_ready() {
                        rewind.cooldown + config.seconds_per_rewind_frame
                    } else {
                        rewind.cooldown - time.delta_seconds()
                    }
                }
            };
            *current_state = CurrentState::Rewinding;
        } else {
            if CurrentState::Rewinding == *current_state {
                history.force_key_frame = true;
            }
            *current_state = CurrentState::Running;
        }

        for (tint) in (&mut tints).join() {
            tint.0 = if *current_state == CurrentState::Running {
                Srgba::new(1.0, 1.0, 1.0, 1.0)
            } else {
                // let (r, g, b, a) = tint.0.into_components();
                // let colour = 0.299 * r + 0.587 * g + 0.114 * b;
                Srgba::new(0.1, 0.1, 0.1, 1.0)
                // Srgba::new(0.299*, 0.587, 0.114, 1.0)
            };
        }
    }
}

pub struct RewindSystem;

impl<'s> System<'s> for RewindSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, DiscretePos>,
        WriteStorage<'s, Steering>,
        ReadStorage<'s, PlayerTag>,
        Read<'s, Rewind>,
        Write<'s, History>,
    );

    fn run(&mut self, (mut transforms, mut positions, mut steerings, player_tags, rewind, mut history): Self::SystemData) {
        if rewind.is_ready() {
            if let Some(frame) = history.pop_frame() {
                println!("Rewinding player to {:?}", frame);
                for (_, transform, pos, steering) in (&player_tags, &mut transforms, &mut positions, &mut steerings).join() {
                    transform.set_translation_x(
                        frame.player_position.x as f32 * 50. + 50.,
                    );
                    *pos = frame.player_position;
                    steering.destination = frame.player_position;
                    // transform.set_translation_y(
                    //     frame.player_pos.y * 50 + 50),
                    // );
                }
            }
        }
    }
}