use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct UiFocusState {
    pub wants_keyboard_input: bool,
}