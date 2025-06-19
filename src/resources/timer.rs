use bevy::prelude::*;

#[derive(Resource)]
pub struct LevelTimer {
    pub timer: Timer,
    pub elapsed: f32,
}

impl Default for LevelTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            elapsed: 0.0,
        }
    }
}

impl LevelTimer {
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.timer.reset();
    }

    pub fn tick(&mut self, delta: std::time::Duration) {
        self.timer.tick(delta);
        self.elapsed += delta.as_secs_f32();
    }

    pub fn get_elapsed_seconds(&self) -> f32 {
        self.elapsed
    }
}