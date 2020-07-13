use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct DebugConfig {
    pub speed_presets: Vec<f32>,
    pub player_speed: f32,
    pub seconds_per_rewind_frame: f32,
    pub skip_straight_to_editor: bool,
}

impl DebugConfig {
    pub fn increase_speed(&mut self) -> (f32, f32) {
        let old_speed = self.player_speed;
        let new_speed = self
            .speed_presets
            .iter()
            .find(|&&speed| speed > self.player_speed);
        if let Some(new_speed) = new_speed {
            self.player_speed = *new_speed;
            (old_speed, self.player_speed)
        } else {
            (self.player_speed, self.player_speed)
        }
    }

    pub fn decrease_speed(&mut self) -> (f32, f32) {
        let old_speed = self.player_speed;
        let new_speed = self
            .speed_presets
            .iter()
            .rev()
            .find(|&&speed| speed < self.player_speed);
        if let Some(new_speed) = new_speed {
            self.player_speed = *new_speed;
            (old_speed, self.player_speed)
        } else {
            (self.player_speed, self.player_speed)
        }
    }
}
