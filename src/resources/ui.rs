use bevy::prelude::*;
use crate::structs::controls::Instruction;

#[derive(Resource, Default)]
pub struct UiFocusState {
    pub wants_keyboard_input: bool,
}

#[derive(Resource, Default)]
pub struct DragDropState {
    pub is_dragging: bool,
    pub dragged_instruction: Option<Instruction>,
    pub start_position: Option<bevy_egui::egui::Vec2>, // Utiliser egui::Vec2 au lieu de bevy::Vec2
}

#[derive(Clone)]
pub struct FunctionSnapshot {
    pub functions: Vec<Vec<Instruction>>,
    pub timestamp: f64,
}

#[derive(Resource)]
pub struct InstructionHistory {
    pub history: Vec<FunctionSnapshot>,
    pub max_history: usize,
}

impl Default for InstructionHistory {
    fn default() -> Self {
        Self {
            history: Vec::new(),
            max_history: 20, // Garde les 20 dernières actions
        }
    }
}

impl InstructionHistory {
    pub fn push(&mut self, snapshot: FunctionSnapshot) {
        self.history.push(snapshot);
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    pub fn pop(&mut self) -> Option<FunctionSnapshot> {
        // Garde au moins un état
        if self.history.len() > 1 {
            self.history.pop()
        } else {
            None
        }
    }

    pub fn get_last(&self) -> Option<&FunctionSnapshot> {
        self.history.last()
    }
}