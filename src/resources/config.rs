use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct DebugConfig {
    pub time_scale_presets: Vec<f32>,
    pub time_scale: f32,
    pub player_speed: f32,
    pub seconds_per_rewind_frame: f32,
    pub skip_straight_to_editor: bool,
}

impl DebugConfig {
    /// Increase the time scale. Everything in the world will move more quickly.
    /// Return a tuple containing the old scale and the new scale.
    /// If the time is already operating at the fastest speed, the time scale will not change.
    pub fn increase_speed(&mut self) -> (f32, f32) {
        let old_time_scale = self.time_scale;
        let new_time_scale = self
            .time_scale_presets
            .iter()
            .find(|&&scale| scale > self.time_scale);
        if let Some(new_time_scale) = new_time_scale {
            self.time_scale = *new_time_scale;
            (old_time_scale, self.time_scale)
        } else {
            (self.time_scale, self.time_scale)
        }
    }

    /// Decrease the time scale. Everything in the world will move more slowly.
    /// Return a tuple containing the old scale and the new scale.
    /// If the time is already operating at the slowest speed, the time scale will not change.
    pub fn decrease_speed(&mut self) -> (f32, f32) {
        let old_time_scale = self.time_scale;
        let new_time_scale = self
            .time_scale_presets
            .iter()
            .rev()
            .find(|&&scale| scale < self.time_scale);
        if let Some(new_time_scale) = new_time_scale {
            self.time_scale = *new_time_scale;
            (old_time_scale, self.time_scale)
        } else {
            (self.time_scale, self.time_scale)
        }
    }
}
