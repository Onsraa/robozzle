use bevy::prelude::*;

#[derive(Clone)]
pub enum PlayerField {
    FirstName,
    LastName,
}

#[derive(Component)]
pub struct PlayerInfoInput {
    field: PlayerField,
}