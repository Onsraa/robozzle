use bevy::prelude::*;

#[derive(Event)]
pub struct SwitchLevelEvent(pub usize);

#[derive(Event)]
pub struct StarCollectedEvent {
    pub x: i32,
    pub y: i32,
}
