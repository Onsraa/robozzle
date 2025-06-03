use bevy::prelude::*;

#[derive(Resource)]
pub struct ExecutionEngine {
    timer: Timer,
    current_function: usize,
    current_instruction: usize,
    call_stack: Vec<(usize, usize)>,
    is_executing: bool,
    is_paused: bool,
    execution_speed: ExecutionSpeed,
    error_message: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ExecutionSpeed {
    Normal,  // x1 - 1.0 seconde
    Fast,    // x2 - 0.5 seconde  
    VeryFast, // x5 - 0.2 seconde
}

impl ExecutionSpeed {
    pub fn get_duration(&self) -> f32 {
        match self {
            ExecutionSpeed::Normal => 1.0,
            ExecutionSpeed::Fast => 0.5,
            ExecutionSpeed::VeryFast => 0.2,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            ExecutionSpeed::Normal => ExecutionSpeed::Fast,
            ExecutionSpeed::Fast => ExecutionSpeed::VeryFast,
            ExecutionSpeed::VeryFast => ExecutionSpeed::Normal,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            ExecutionSpeed::Normal => "x1",
            ExecutionSpeed::Fast => "x2",
            ExecutionSpeed::VeryFast => "x5",
        }
    }
}

impl ExecutionEngine {
    pub fn new() -> Self {
        let speed = ExecutionSpeed::Normal;
        Self {
            timer: Timer::from_seconds(speed.get_duration(), TimerMode::Repeating),
            current_function: 0,
            current_instruction: 0,
            call_stack: Vec::new(),
            is_executing: false,
            is_paused: false,
            execution_speed: speed,
            error_message: None,
        }
    }

    pub fn start_execution(&mut self) {
        self.is_executing = true;
        self.is_paused = false;
        self.current_function = 0;
        self.current_instruction = 0;
        self.call_stack.clear();
        self.error_message = None;
        self.timer.reset();
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
    }

    pub fn stop(&mut self) {
        self.is_executing = false;
        self.is_paused = false;
        self.current_function = 0;
        self.current_instruction = 0;
        self.call_stack.clear();
        self.error_message = None;
    }

    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.stop();
    }

    pub fn get_error(&self) -> Option<&String> {
        self.error_message.as_ref()
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    pub fn change_speed(&mut self) {
        self.execution_speed = self.execution_speed.next();
        // Met à jour le timer avec la nouvelle vitesse
        self.timer = Timer::from_seconds(
            self.execution_speed.get_duration(),
            TimerMode::Repeating
        );
    }

    pub fn get_speed(&self) -> ExecutionSpeed {
        self.execution_speed
    }

    pub fn is_executing(&self) -> bool {
        self.is_executing && !self.is_paused
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn is_stopped(&self) -> bool {
        !self.is_executing
    }

    pub fn tick(&mut self, delta: std::time::Duration) -> bool {
        if self.is_executing() {
            self.timer.tick(delta);
            self.timer.just_finished()
        } else {
            false
        }
    }

    // Getters pour l'exécution
    pub fn get_current_function(&self) -> usize {
        self.current_function
    }

    pub fn get_current_instruction(&self) -> usize {
        self.current_instruction
    }

    pub fn advance_instruction(&mut self) {
        self.current_instruction += 1;
    }

    pub fn call_function(&mut self, function_id: usize) {
        // Sauvegarde l'état actuel sur la pile
        self.call_stack.push((self.current_function, self.current_instruction));
        // Change vers la nouvelle fonction
        self.current_function = function_id;
        self.current_instruction = 0;
    }

    pub fn return_from_function(&mut self) -> bool {
        if let Some((prev_function, prev_instruction)) = self.call_stack.pop() {
            self.current_function = prev_function;
            self.current_instruction = prev_instruction + 1; // Continue après l'appel
            true
        } else {
            // Plus de fonctions dans la pile, arrêt de l'exécution
            self.stop();
            false
        }
    }
}