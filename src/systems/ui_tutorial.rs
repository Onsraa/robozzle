use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use crate::resources::level::{LevelManager, LevelType};
use crate::events::level::SwitchLevelEvent;
use crate::states::game::GameState;

// Système pour l'interface du tutoriel
pub fn tutorial_ui_system(
    mut contexts: EguiContexts,
    mut level_manager: ResMut<LevelManager>,
    mut level_switch_events: EventWriter<SwitchLevelEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let ctx = contexts.ctx_mut();

    // Panel pour afficher la progression du tutoriel
    egui::TopBottomPanel::top("tutorial_progress")
        .min_height(60.0)
        .show(ctx, |ui| {
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_space(20.0);

                // Titre
                ui.heading("🎓 Mode Tutoriel");

                ui.add_space(40.0);

                // Progression
                let current_level_id = level_manager
                    .get_current_level()
                    .map(|level| level.id)
                    .unwrap_or(0);
                let total_tutorials = level_manager.get_levels_count();

                ui.label(format!("Étape {} sur {}", current_level_id + 1, total_tutorials));

                ui.add_space(40.0);

                // Indicateur de complétion
                if let Some(current_level) = level_manager.get_current_level() {
                    if let Some(state) = level_manager.get_problem_state(current_level.id) {
                        if state.is_completed {
                            ui.colored_label(
                                egui::Color32::from_rgb(80, 200, 80),
                                "✓ Niveau complété!"
                            );
                        } else {
                            ui.label(format!("⭐ {}/{} étoiles collectées",
                                             state.stars_collected, current_level.total_stars));
                        }
                    }
                }

                ui.add_space(40.0);

                // Bouton suivant (si niveau complété)
                if level_manager.can_proceed_to_next() {
                    if ui.button("➡️ Suivant").clicked() {
                        if let Some(next_id) = level_manager.try_next_level() {
                            level_switch_events.send(SwitchLevelEvent(next_id));
                        }
                    }
                }

                // Vérifier si tous les tutoriels sont complétés
                if level_manager.are_all_tutorials_completed() {
                    ui.add_space(20.0);
                    if ui.add_sized(
                        [150.0, 40.0],
                        egui::Button::new("✅ Continuer")
                            .fill(egui::Color32::from_rgb(80, 200, 80))
                    ).clicked() {
                        next_state.set(GameState::PlayerInfo);
                    }
                }
            });

            ui.add_space(10.0);
        });

    // Message d'aide si le niveau n'est pas complété
    if !level_manager.can_proceed_to_next() {
        egui::Window::new("Instructions")
            .anchor(egui::Align2::RIGHT_TOP, [-20.0, 80.0])
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label("📌 Collectez toutes les étoiles pour passer au niveau suivant!");

                if let Some(level) = level_manager.get_current_level() {
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Afficher les instructions spécifiques selon le niveau
                    match level.id {
                        0 => {
                            ui.heading("Instructions de base");
                            ui.label("• Utilisez les flèches pour diriger le robot");
                            ui.label("• Le robot doit collecter toutes les étoiles ⭐");
                            ui.label("• Cliquez sur 'Start' pour lancer l'exécution");
                        }
                        1 => {
                            ui.heading("Les fonctions");
                            ui.label("• Les fonctions permettent de réutiliser du code");
                            ui.label("• F1 est la fonction principale");
                            ui.label("• Utilisez F2 pour créer une sous-routine");
                            ui.label("• Appelez F2 depuis F1 avec le bouton F2");
                        }
                        2 => {
                            ui.heading("Les conditions de couleur");
                            ui.label("• Le robot peut tester la couleur de la case");
                            ui.label("• Cliquez sur une couleur puis sur une instruction");
                            ui.label("• L'instruction ne s'exécute que sur cette couleur");
                        }
                        _ => {
                            ui.label("Complétez ce niveau pour continuer.");
                        }
                    }
                }
            });
    }
}