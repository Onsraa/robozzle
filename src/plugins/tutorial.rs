use bevy::prelude::*;
use bevy_egui::EguiContextPass;
use crate::states::game::GameState;
use crate::systems::ui_tutorial::tutorial_ui_system;

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                EguiContextPass,
                tutorial_ui_system.run_if(in_state(GameState::Tutorial)),
            );
    }
}