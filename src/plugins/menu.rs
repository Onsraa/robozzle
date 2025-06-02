use crate::states::game::GameState;
use crate::systems::level::{auto_start_first_level_system, display_level_info_system, display_controls_system};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Menu),
                auto_start_first_level_system,
            )
            .add_systems(
                Update,
                (
                    display_level_info_system,
                    display_controls_system,
                ).run_if(in_state(GameState::Editing)),
            );
    }
}