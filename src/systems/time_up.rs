use crate::resources::level::LevelManager;
use crate::resources::player::PlayerInfo;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

// Système pour l'interface de fin de jeu
pub fn time_up_ui_system(
    mut contexts: EguiContexts,
    level_manager: Res<LevelManager>,
    player_info: Res<PlayerInfo>,
    mut exit: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();

    // Vérifier si tous les puzzles sont complétés
    let all_completed = level_manager.are_all_levels_completed();

    // Fenêtre centrée pour le message de fin
    egui::Window::new("Fin du test")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .resizable(false)
        .collapsible(false)
        .default_width(500.0)
        .show(ctx, |ui| {
            ui.set_min_width(450.0);
            ui.vertical_centered(|ui| {
                if all_completed {
                    ui.add_space(20.0);
                    ui.heading("Félicitations!");
                    ui.add_space(20.0);
                    ui.label(
                        egui::RichText::new("Vous avez réussi tous les puzzles!")
                            .size(20.0)
                            .color(egui::Color32::from_rgb(80, 200, 80)),
                    );
                } else {
                    ui.heading("Temps écoulé");
                    ui.add_space(30.0);
                    ui.label(egui::RichText::new("Merci pour votre participation!").size(20.0));
                }

                ui.add_space(30.0);
                ui.separator();
                ui.add_space(20.0);

                // Résumé des performances
                ui.heading("📊 Résumé");
                ui.add_space(10.0);

                let levels = level_manager.get_levels();
                let mut completed_count = 0;
                let mut total_stars_collected = 0;
                let mut total_stars_available = 0;

                for level in levels {
                    if let Some(state) = level_manager.get_problem_state(level.id) {
                        if state.is_completed {
                            completed_count += 1;
                        }
                        total_stars_collected += state.stars_collected;
                        total_stars_available += level.total_stars;
                    }
                }

                ui.label(format!(
                    "Candidat: {} {}",
                    player_info.last_name, player_info.first_name
                ));
                ui.label(format!(
                    "Puzzles complétés: {}/{}",
                    completed_count,
                    levels.len()
                ));
                ui.label(format!(
                    "Étoiles collectées: {}/{}",
                    total_stars_collected, total_stars_available
                ));

                ui.add_space(30.0);

                // Détail par niveau
                ui.collapsing("Détails par niveau", |ui| {
                    for level in levels {
                        if let Some(state) = level_manager.get_problem_state(level.id) {
                            ui.horizontal(|ui| {
                                let status_icon = if state.is_completed { "✅" } else { "❌" };
                                ui.label(format!("{} {}", status_icon, level.name));

                                ui.add_space(10.0);

                                ui.label(format!(
                                    "⭐ {}/{}",
                                    state.stars_collected, level.total_stars
                                ));

                                if let Some(time) = state.completion_time {
                                    ui.add_space(10.0);
                                    ui.label(format!("⏱️ {:.1}s", time));
                                }
                            });
                        }
                    }
                });

                ui.add_space(30.0);

                if ui.button("Quitter").clicked() {
                    // Sauvegarder le rapport avant de quitter
                    if let Err(e) = level_manager.save_final_report(&player_info) {
                        error!("Erreur lors de la sauvegarde du rapport: {}", e);
                    } else {
                        info!("Rapport sauvegardé avec succès");
                    }

                    // Fermer l'application
                    exit.write(AppExit::Success);
                }
            });
        });
}

// Plugin pour l'interface de fin
pub struct TimeUpPlugin;

impl Plugin for TimeUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            bevy_egui::EguiContextPass,
            time_up_ui_system.run_if(in_state(crate::states::game::GameState::TimeUp)),
        );
    }
}
