use crate::states::game::GameState;
use crate::systems::player::{
    cleanup_player_info_ui, handle_input_focus, handle_player_info_validation, setup_player_info_ui,
};
use bevy::prelude::*;
use bevy_simple_text_input::TextInputPlugin;

pub struct PlayerInfoPlugin;

impl Plugin for PlayerInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextInputPlugin)
            .add_systems(OnEnter(GameState::PlayerInfo), setup_player_info_ui)
            .add_systems(
                Update,
                handle_player_info_validation.run_if(in_state(GameState::PlayerInfo)),
            )
            .add_systems(
                Update,
                handle_input_focus.run_if(in_state(GameState::PlayerInfo)),
            )
            .add_systems(OnExit(GameState::PlayerInfo), cleanup_player_info_ui);
    }
}
