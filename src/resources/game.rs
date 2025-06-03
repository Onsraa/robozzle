use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTimer {
    timer: Timer,
    total_duration: f32,  // en secondes
    is_finished: bool,
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

    pub fn tick(&mut self, delta: std::time::Duration) {
        if !self.is_finished {
            self.timer.tick(delta);
            if self.timer.finished() {
                self.is_finished = true;
            }
        }
    }

    pub fn just_finished(&self) -> bool {
        self.timer.just_finished()
    }

    pub fn remaining_time(&self) -> f32 {
        if self.is_finished {
            0.0
        } else {
            self.total_duration - self.timer.elapsed_secs()
        }
    }

    pub fn remaining_minutes(&self) -> u32 {
        (self.remaining_time() / 60.0) as u32
    }

    pub fn remaining_seconds(&self) -> u32 {
        (self.remaining_time() % 60.0) as u32
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }
}