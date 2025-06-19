use crate::resources::timer::LevelTimer;
use crate::states::game::GameState;
use crate::systems::timer::*;
use bevy::prelude::*;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelTimer>()
            .add_systems(
                Update,
                (
                    update_game_timer_system,
                    update_level_timer_system,
                    record_completion_time_system,
                    handle_time_up_system,
                )
                    .run_if(in_state(GameState::Editing)),
            )
            .add_systems(
                Update,
                reset_level_timer_system
            );
    }
}