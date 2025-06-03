use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use crate::resources::player::PlayerInfo;
use crate::states::game::GameState;

// Système pour l'interface de saisie du nom et prénom
pub fn player_info_ui_system(
    mut contexts: EguiContexts,
    mut player_info: ResMut<PlayerInfo>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let ctx = contexts.ctx_mut();

    // Fenêtre centrée pour la saisie des informations
    egui::Window::new("Information du candidat")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.heading("Veuillez entrer vos informations");
            ui.add_space(20.0);

            ui.vertical_centered(|ui| {
                // Champ Nom
                ui.horizontal(|ui| {
                    ui.label("Nom : ");
                    ui.add_space(20.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut player_info.last_name)
                            .desired_width(200.0)
                            .hint_text("Entrez votre nom")
                    );
                });

                ui.add_space(10.0);

                // Champ Prénom
                ui.horizontal(|ui| {
                    ui.label("Prénom : ");
                    ui.add_space(5.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut player_info.first_name)
                            .desired_width(200.0)
                            .hint_text("Entrez votre prénom")
                    );
                });

                ui.add_space(20.0);

                // Bouton de validation
                let can_submit = !player_info.first_name.trim().is_empty()
                    && !player_info.last_name.trim().is_empty();

                if ui.add_enabled(
                    can_submit,
                    egui::Button::new("Commencer les exercices")
                        .fill(egui::Color32::from_rgb(80, 200, 80))
                ).clicked() && can_submit {
                    // Nettoyer les espaces
                    player_info.first_name = player_info.first_name.trim().to_string();
                    player_info.last_name = player_info.last_name.trim().to_string();

                    info!("Informations du candidat: {} {}", 
                          player_info.last_name, player_info.first_name);

                    // Passer au chargement des niveaux normaux
                    next_state.set(GameState::Loading);
                }

                if !can_submit {
                    ui.add_space(10.0);
                    ui.colored_label(
                        egui::Color32::from_rgb(200, 80, 80),
                        "Veuillez remplir tous les champs"
                    );
                }
            });
        });
}