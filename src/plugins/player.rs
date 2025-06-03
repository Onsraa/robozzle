use bevy::prelude::*;
use bevy_egui::EguiContextPass;
use crate::states::game::GameState;
use crate::systems::player::player_info_ui_system;

pub struct PlayerInfoPlugin;

impl Plugin for PlayerInfoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                EguiContextPass,
                player_info_ui_system.run_if(in_state(GameState::PlayerInfo)),
            );
    }
}