use crate::resources::loading::LoadingState;
use crate::states::game::GameState;
use crate::systems::loading::*;
use bevy::prelude::*;

pub struct LevelLoadingPlugin;

impl Plugin for LevelLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LoadingState>()
            .add_systems(
                OnEnter(GameState::LoadingTutorial),
                load_tutorial_levels_on_enter_system,
            )
            .add_systems(
                OnEnter(GameState::Loading),
                load_levels_on_enter_system,
            )
            .add_systems(
                Update,
                (
                    handle_loading_error_system
                        .run_if(in_state(GameState::LoadingTutorial)
                            .or(in_state(GameState::Loading))),
                ),
            );
    }
}
