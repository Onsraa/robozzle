use bevy::prelude::*;

#[derive(Component)]
pub struct ProblemButton {
    problem_id: usize,
}

#[derive(Component)]
pub struct ProblemStars {
    problem_id: usize,
    collected: usize,
    total: usize,
}

#[derive(Component)]
pub struct TimerDisplay;