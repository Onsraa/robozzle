use bevy::prelude::*;
use crate::structs::controls::Instruction;

#[derive(Component)]
pub struct ProblemButton {
    pub problem_id: usize,
}

#[derive(Component)]
pub struct ProblemStars {
    pub problem_id: usize,
    pub collected: usize,
    pub total: usize,
}

#[derive(Component)]
pub struct TimerDisplay;

// Composants pour les boutons de contrôle
#[derive(Component)]
pub struct PlayPauseButton;

#[derive(Component)]
pub struct SpeedButton;

#[derive(Component)]
pub struct ResetButton;

#[derive(Component)]
pub struct ClearButton;

// Composants pour l'édition des instructions
#[derive(Component)]
pub struct FunctionEditor {
    pub function_id: usize,
}

#[derive(Component)]
pub struct InstructionSlot {
    pub function_id: usize,
    pub slot_index: usize,
}

// Composant pour tracker la couleur de condition d'un slot
#[derive(Component)]
pub struct SlotConditionColor {
    pub color: Option<Color>,
}

#[derive(Component)]
pub struct InstructionPalette;

#[derive(Component)]
pub struct InstructionButton {
    pub instruction: Instruction,
}

// Nouveau composant pour les boutons de condition de couleur
#[derive(Component)]
pub struct ColorConditionButton {
    pub color: Color,
}

// Composants pour les messages d'état
#[derive(Component)]
pub struct ErrorMessage;

#[derive(Component)]
pub struct StatusMessage;

// Composant pour l'interface d'édition complète
#[derive(Component)]
pub struct InstructionInterface;