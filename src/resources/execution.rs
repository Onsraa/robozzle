use bevy::prelude::*;

#[derive(Resource)]
pub struct ExecutionEngine {
    timer: Timer,
    current_function: usize,
    current_instruction: usize,
    call_stack: Vec<(usize, usize)>,            
    is_executing: bool,
    execution_speed: f32,
}

impl ExecutionEngine {
    pub fn new(execution_speed: f32) -> Self {
        Self {
            timer: Timer::from_seconds(execution_speed, TimerMode::Repeating),
            current_function: 0,
            current_instruction: 0,
            call_stack: Vec::new(),
            is_executing: false,
            execution_speed,
        }
    }

    pub fn start_execution(&mut self) {
        self.is_executing = true;
        self.current_function = 0;
        self.current_instruction = 0;
        self.call_stack.clear();
        self.timer.reset();
    }

    pub fn pause(&mut self) {
        self.is_executing = false;
    }

    pub fn resume(&mut self) {
        self.is_executing = true;
    }

    pub fn stop(&mut self) {
        self.is_executing = false;
        self.current_function = 0;
        self.current_instruction = 0;
        self.call_stack.clear();
    }
}