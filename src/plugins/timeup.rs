use bevy::prelude::*;
use crate::systems::time_up::time_up_ui_system;

pub struct TimeUpPlugin;

impl Plugin for TimeUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            bevy_egui::EguiContextPass,
            time_up_ui_system.run_if(in_state(crate::states::game::GameState::TimeUp)),
        );
    }
}