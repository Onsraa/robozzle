use bevy::prelude::*;

#[derive(Event)]
pub struct StartExecutionEvent;

#[derive(Event)]
pub struct PauseExecutionEvent;

#[derive(Event)]
pub struct StopExecutionEvent;