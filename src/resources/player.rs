use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerInfo {
    pub first_name: String,
    pub last_name: String,
}