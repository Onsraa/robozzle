use bevy::prelude::*;
use crate::structs::controls::Instruction;

#[derive(Resource, Default)]
pub struct DragDropState {
    pub is_dragging: bool,
    pub dragged_instruction: Option<Instruction>,
}