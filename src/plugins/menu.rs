use crate::states::game::GameState;
use crate::systems::level::{auto_start_first_level_system, cleanup_current_level};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), auto_start_first_level_system)
            .add_systems(OnEnter(GameState::Tutorial), auto_start_first_level_system)
            .add_systems(OnExit(GameState::Tutorial), cleanup_current_level);
    }
}
