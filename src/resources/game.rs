use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTimer {
    pub timer: Timer,
    pub total_duration: f32,  // en seconde
    pub is_finished: bool,
}

impl GameTimer {
    pub fn new(duration_minutes: f32) -> Self {
        let duration = duration_minutes * 60.0;
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            total_duration: duration,
            is_finished: false,
        }
    }

    pub fn remaining_time(&self) -> f32 {
        self.total_duration - self.timer.elapsed_secs()
    }

    pub fn remaining_minutes(&self) -> u32 {
        (self.remaining_time() / 60.0) as u32
    }

    pub fn remaining_seconds(&self) -> u32 {
        (self.remaining_time() % 60.0) as u32
    }
}