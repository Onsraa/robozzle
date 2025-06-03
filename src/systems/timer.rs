use bevy::prelude::*;
use crate::resources::game::GameTimer;
use crate::resources::timer::LevelTimer;
use crate::resources::level::{LevelManager, LevelType};
use crate::events::game::TimeUpEvent;
use crate::events::level::SwitchLevelEvent;
use crate::states::game::GameState;

// Système pour mettre à jour le timer global
pub fn update_game_timer_system(
    time: Res<Time>,
    mut game_timer: ResMut<GameTimer>,
    mut time_up_events: EventWriter<TimeUpEvent>,
    level_manager: Res<LevelManager>,
) {
    // Ne compte le temps que pour les niveaux normaux
    if level_manager.get_current_level_type() == LevelType::Normal {
        game_timer.tick(time.delta());

        if game_timer.just_finished() {
            // Le temps est écoulé
            time_up_events.write(TimeUpEvent);
            info!("Temps écoulé!");
        }
    }
}

// Système pour mettre à jour le timer du niveau actuel
pub fn update_level_timer_system(
    time: Res<Time>,
    mut level_timer: ResMut<LevelTimer>,
    level_manager: Res<LevelManager>,
) {
    // Timer actif seulement pour les niveaux normaux
    if level_manager.get_current_level_type() == LevelType::Normal {
        level_timer.tick(time.delta());
    }
}

// Système pour enregistrer le temps de complétion
pub fn record_completion_time_system(
    level_timer: Res<LevelTimer>,
    mut level_manager: ResMut<LevelManager>,
) {
    if let Some(current_level) = level_manager.get_current_level() {
        let level_id = current_level.id;

        if let Some(problem_state) = level_manager.get_problem_state_mut(level_id) {
            if problem_state.is_completed && !problem_state.completion_time_recorded {
                problem_state.set_completion_time(level_timer.get_elapsed_seconds());
                problem_state.completion_time_recorded = true;
                info!("Temps enregistré pour le niveau {}: {:.1}s", 
                      level_id + 1, level_timer.get_elapsed_seconds());
            }
        }
    }
}

// Système pour réinitialiser le timer quand on change de niveau
pub fn reset_level_timer_system(
    mut level_timer: ResMut<LevelTimer>,
    mut switch_events: EventReader<SwitchLevelEvent>,
) {
    // Reset le timer seulement si on a un événement de changement de niveau
    for event in switch_events.read() {
        level_timer.reset();
        info!("Timer réinitialisé pour le niveau {}", event.0 + 1);
    }
}

// Système pour gérer l'événement TimeUp
pub fn handle_time_up_system(
    mut time_up_events: EventReader<TimeUpEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in time_up_events.read() {
        next_state.set(GameState::TimeUp);
    }
}