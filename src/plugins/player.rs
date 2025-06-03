use crate::states::game::GameState;
use crate::systems::player::{
    cleanup_player_info_ui, handle_player_info_validation,
    handle_submit_events, setup_player_info_ui,
};
use bevy::prelude::*;
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};

pub struct PlayerInfoPlugin;

impl Plugin for PlayerInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextInputPlugin)
            .add_systems(OnEnter(GameState::PlayerInfo), setup_player_info_ui)
            .add_systems(
                Update,
                (
                    handle_player_info_validation,
                    handle_submit_events.after(TextInputSystem),
                ).run_if(in_state(GameState::PlayerInfo)),
            )
            .add_systems(OnExit(GameState::PlayerInfo), cleanup_player_info_ui);
    }
}