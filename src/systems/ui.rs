use crate::components::grid::Grid;
use crate::components::level::CurrentLevel;
use crate::components::robot::Robot;
use crate::events::level::SwitchLevelEvent;
use crate::resources::execution::ExecutionEngine;
use crate::resources::grid::GridDisplayConfig;
use crate::resources::level::LevelManager;
use crate::states::game::GameState;
use crate::structs::controls::Instruction;
use crate::structs::tile::TileColor;
use bevy::prelude::*;
use bevy_egui::{EguiContextPass, EguiContexts, EguiPlugin, egui};

// Resource pour l'état d'édition avec egui
#[derive(Resource, Default)]
pub struct EguiEditState {
    pub selected_instruction: Option<Instruction>,
    pub selected_condition: Option<TileColor>,
}

// Resource pour stocker les textures des instructions
#[derive(Resource, Default)]
pub struct InstructionTextures {
    pub forward: Option<egui::TextureHandle>,
    pub turn_left: Option<egui::TextureHandle>,
    pub turn_right: Option<egui::TextureHandle>,
}

// Système pour charger les textures au démarrage
pub fn load_instruction_textures(
    mut contexts: EguiContexts,
    mut textures: ResMut<InstructionTextures>,
    asset_server: Res<AssetServer>,
) {
    let ctx = contexts.ctx_mut();

    // Charge les images seulement si pas déjà chargées
    if textures.forward.is_none() {
        // Charger l'image straight.png
        if let Ok(image_data) = std::fs::read("assets/images/straight.png") {
            if let Ok(image) = image::load_from_memory(&image_data) {
                let size = [image.width() as _, image.height() as _];
                let rgba = image.to_rgba8();
                let pixels = rgba.as_flat_samples();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                textures.forward =
                    Some(ctx.load_texture("forward", color_image, Default::default()));
            }
        }
    }

    if textures.turn_left.is_none() {
        // Charger l'image left.png
        if let Ok(image_data) = std::fs::read("assets/images/left.png") {
            if let Ok(image) = image::load_from_memory(&image_data) {
                let size = [image.width() as _, image.height() as _];
                let rgba = image.to_rgba8();
                let pixels = rgba.as_flat_samples();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                textures.turn_left =
                    Some(ctx.load_texture("turn_left", color_image, Default::default()));
            }
        }
    }

    if textures.turn_right.is_none() {
        // Charger l'image right.png
        if let Ok(image_data) = std::fs::read("assets/images/right.png") {
            if let Ok(image) = image::load_from_memory(&image_data) {
                let size = [image.width() as _, image.height() as _];
                let rgba = image.to_rgba8();
                let pixels = rgba.as_flat_samples();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                textures.turn_right =
                    Some(ctx.load_texture("turn_right", color_image, Default::default()));
            }
        }
    }
}

// Système principal d'interface egui
pub fn ui_system(
    mut contexts: EguiContexts,
    mut edit_state: ResMut<EguiEditState>,
    mut level_manager: ResMut<LevelManager>,
    mut execution_engine: ResMut<ExecutionEngine>,
    mut robot_query: Query<&mut Robot, With<CurrentLevel>>,
    mut grid_query: Query<&mut Grid, With<CurrentLevel>>,
    textures: Res<InstructionTextures>,
    mut level_switch_events: EventWriter<SwitchLevelEvent>,
    mut display_config: ResMut<GridDisplayConfig>,
    game_timer: Res<crate::resources::game::GameTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let ctx = contexts.ctx_mut();

    // Panel de gauche pour la sélection des niveaux (toujours affiché maintenant)
    let panel_width = 200.0;
    egui::SidePanel::left("level_selector")
        .default_width(panel_width)
        .show(ctx, |ui| {
            ui.add_space(8.0);

            // Titre selon le mode
            if level_manager.get_current_level_type()
                == crate::resources::level::LevelType::Tutorial
            {
                ui.heading("🎓 Tutoriel");
            } else {
                ui.heading("📋 Niveaux");
            }
            ui.separator();

            // Chronomètre seulement en mode normal
            if level_manager.get_current_level_type() == crate::resources::level::LevelType::Normal
            {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    let remaining_mins = game_timer.remaining_minutes();
                    let remaining_secs = game_timer.remaining_seconds();

                    let time_color = if remaining_mins < 5 {
                        egui::Color32::from_rgb(200, 80, 80)
                    } else if remaining_mins < 10 {
                        egui::Color32::from_rgb(200, 200, 80)
                    } else {
                        egui::Color32::from_rgb(80, 200, 80)
                    };

                    ui.label(
                        egui::RichText::new(format!(
                            "⏱️ {:02}:{:02}",
                            remaining_mins, remaining_secs
                        ))
                        .size(18.0)
                        .color(time_color)
                        .strong(),
                    );
                });
                ui.separator();
            }
            ui.add_space(8.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                let current_level_id = level_manager
                    .get_current_level()
                    .map(|level| level.id)
                    .unwrap_or(0);

                let levels_count = level_manager.get_levels_count();

                for i in 0..levels_count {
                    if let Some(level) = level_manager.get_levels().get(i) {
                        let is_current = i == current_level_id;
                        let is_completed = level_manager
                            .get_problem_state(i)
                            .map(|state| state.is_completed)
                            .unwrap_or(false);

                        ui.horizontal(|ui| {
                            ui.add_space(5.0);

                            // Indicateur de niveau actuel centré
                            ui.allocate_ui_with_layout(
                                egui::Vec2::new(15.0, 25.0),
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    if is_current {
                                        ui.label("▶");
                                    }
                                },
                            );

                            // Bouton du niveau
                            let button_text = format!("{}", level.name);
                            let button = egui::Button::new(button_text);

                            let button = if is_current {
                                button.fill(egui::Color32::from_rgb(100, 150, 200))
                            } else if is_completed {
                                button.fill(egui::Color32::from_rgb(80, 180, 80))
                            } else {
                                button.fill(egui::Color32::from_gray(100))
                            };

                            // En mode tutoriel, on ne peut pas changer de niveau si pas complété
                            let can_switch = if level_manager.get_current_level_type()
                                == crate::resources::level::LevelType::Tutorial
                            {
                                i <= current_level_id
                                    || (i == current_level_id + 1
                                        && level_manager.can_proceed_to_next())
                            } else {
                                true
                            };

                            if ui.add_sized([110.0, 25.0], button).clicked() && can_switch {
                                if i != current_level_id {
                                    level_switch_events.send(SwitchLevelEvent(i));
                                    execution_engine.stop();
                                }
                            }

                            // Étoiles collectées
                            if let Some(state) = level_manager.get_problem_state(i) {
                                ui.label(format!(
                                    "⭐{}/{}",
                                    state.stars_collected, level.total_stars
                                ));
                            }
                        });

                        ui.add_space(5.0);
                    }
                }

                ui.add_space(8.0);

                // Bouton "Suivant" si tous les tutoriels sont complétés
                if level_manager.get_current_level_type()
                    == crate::resources::level::LevelType::Tutorial
                    && level_manager.are_all_tutorials_completed()
                {
                    ui.separator();
                    ui.add_space(8.0);

                    ui.vertical_centered(|ui| {
                        if ui
                            .add_sized(
                                [150.0, 40.0],
                                egui::Button::new("➡️ Suivant")
                                    .fill(egui::Color32::from_rgb(80, 200, 80)),
                            )
                            .clicked()
                        {
                            next_state.set(GameState::PlayerInfo);
                        }
                    });

                    ui.add_space(8.0);
                }
            });
        });

    // Mettre à jour la configuration d'affichage avec la largeur du panel
    display_config.left_panel_width = panel_width;

    // Calcul de la taille de la fenêtre principale (réduite)
    let window_width = (ctx.screen_rect().width() * 0.35).min(700.0); // Réduit de 40% à 35%
    let num_functions = level_manager
        .get_current_level()
        .map(|level| level.function_limits.len())
        .unwrap_or(1);
    let function_height = 50.0 * num_functions as f32 + 30.0; // Réduit l'espace pour les fonctions
    let window_height = 180.0 + function_height; // Hauteur totale réduite

    // Position centrée en bas et collée au bord
    let x_pos = (ctx.screen_rect().width() - window_width) / 2.0;
    let y_pos = ctx.screen_rect().height() - window_height;

    // Fenêtre principale fixe
    egui::Window::new("Robozzle")
        .title_bar(false)
        .resizable(false)
        .collapsible(false)
        .fixed_pos([x_pos, y_pos])
        .fixed_size([window_width, window_height])
        .show(ctx, |ui| {
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_space(10.0);

                // Partie gauche - Instructions et Fonctions
                ui.vertical(|ui| {
                    let available_width = window_width - 240.0;
                    ui.set_min_width(available_width);
                    ui.set_max_width(available_width);

                    // Section Instructions (en haut)
                    ui.group(|ui| {
                        ui.set_min_height(85.0); // Hauteur réduite
                        ui.add_space(8.0);

                        ui.vertical(|ui| {
                            // Première ligne : instructions de mouvement
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);

                                // Boutons avec images (taille réduite)
                                instruction_image_button(
                                    ui,
                                    &mut edit_state,
                                    &textures.forward,
                                    "Avancer",
                                    Instruction::Forward,
                                );
                                ui.add_space(3.0);
                                instruction_image_button(
                                    ui,
                                    &mut edit_state,
                                    &textures.turn_left,
                                    "Tourner à gauche",
                                    Instruction::TurnLeft,
                                );
                                ui.add_space(3.0);
                                instruction_image_button(
                                    ui,
                                    &mut edit_state,
                                    &textures.turn_right,
                                    "Tourner à droite",
                                    Instruction::TurnRight,
                                );

                                ui.add_space(15.0);

                                // Boutons de fonction
                                if let Some(level) = level_manager.get_current_level() {
                                    for i in 0..level.function_limits.len().min(4) {
                                        let label = format!("F{}", i + 1);
                                        instruction_text_button(
                                            ui,
                                            &mut edit_state,
                                            &label,
                                            &format!("Appeler F{}", i + 1),
                                            Instruction::CallFunction(i),
                                            18.0, // Taille de police légèrement réduite
                                        );
                                        ui.add_space(3.0);
                                    }
                                }
                            });

                            ui.add_space(8.0);

                            // Deuxième ligne : conditions de couleur
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                condition_button(
                                    ui,
                                    &mut edit_state,
                                    "",
                                    TileColor::Green,
                                    egui::Color32::from_rgb(80, 200, 80),
                                    40.0,
                                ); // Taille réduite
                                ui.add_space(3.0);
                                condition_button(
                                    ui,
                                    &mut edit_state,
                                    "",
                                    TileColor::Red,
                                    egui::Color32::from_rgb(200, 80, 80),
                                    40.0,
                                );
                                ui.add_space(3.0);
                                condition_button(
                                    ui,
                                    &mut edit_state,
                                    "",
                                    TileColor::Blue,
                                    egui::Color32::from_rgb(80, 80, 200),
                                    40.0,
                                );
                            });
                        });

                        ui.add_space(8.0);
                    });

                    ui.add_space(10.0);

                    // Section Fonctions (en bas)
                    ui.group(|ui| {
                        ui.add_space(8.0);
                        function_editor_ui(
                            ui,
                            &mut edit_state,
                            &mut level_manager,
                            &textures,
                            &execution_engine,
                        );
                        ui.add_space(8.0);
                    });
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // Partie droite - Contrôles d'exécution
                ui.vertical(|ui| {
                    ui.set_min_width(200.0);
                    ui.set_max_width(200.0);

                    // Informations du niveau
                    if let Some(level) = level_manager.get_current_level() {
                        ui.group(|ui| {
                            ui.add_space(3.0);
                            ui.label(egui::RichText::new(&level.name).strong().size(14.0));
                            if let Some(state) = level_manager.get_problem_state(level.id) {
                                ui.label(format!(
                                    "⭐ {}/{}",
                                    state.stars_collected, level.total_stars
                                ));
                            }
                            ui.add_space(3.0);
                        });
                    }

                    ui.add_space(10.0);

                    // Bouton Start/Pause
                    let (text, color) = if execution_engine.is_stopped() {
                        ("Start", egui::Color32::from_rgb(80, 200, 80))
                    } else if execution_engine.is_paused() {
                        ("Resume", egui::Color32::from_rgb(80, 150, 200))
                    } else {
                        ("Pause", egui::Color32::from_rgb(200, 200, 80))
                    };

                    if ui
                        .add_sized(
                            [180.0, 40.0], // Taille réduite
                            egui::Button::new(text).fill(color),
                        )
                        .clicked()
                    {
                        if execution_engine.is_stopped() {
                            reset_level_state(
                                &mut robot_query,
                                &mut grid_query,
                                &mut level_manager,
                            );
                            execution_engine.start_execution();
                        } else if execution_engine.is_executing() {
                            execution_engine.pause();
                        } else {
                            execution_engine.resume();
                        }
                    }

                    ui.add_space(8.0);

                    // Boutons Reset et Clear
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(
                                [85.0, 35.0], // Taille réduite
                                egui::Button::new("Reset"),
                            )
                            .clicked()
                        {
                            execution_engine.stop();
                            execution_engine.clear_error();
                            reset_level_state(
                                &mut robot_query,
                                &mut grid_query,
                                &mut level_manager,
                            );
                        }

                        if ui
                            .add_sized([85.0, 35.0], egui::Button::new("Clear"))
                            .clicked()
                        {
                            clear_all_instructions(&mut level_manager);
                            execution_engine.stop();
                        }
                    });

                    ui.add_space(8.0);

                    // Bouton de vitesse
                    let speed_text = match execution_engine.get_speed() {
                        crate::resources::execution::ExecutionSpeed::Normal => "Normal (x1)",
                        crate::resources::execution::ExecutionSpeed::Fast => "Fast (x2)",
                        crate::resources::execution::ExecutionSpeed::VeryFast => "Very Fast (x5)",
                    };

                    if ui
                        .add_sized(
                            [180.0, 30.0], // Taille réduite
                            egui::Button::new(speed_text),
                        )
                        .clicked()
                    {
                        execution_engine.change_speed();
                    }

                    // Message d'erreur
                    if let Some(error) = execution_engine.get_error() {
                        ui.add_space(8.0);
                        ui.colored_label(egui::Color32::RED, error);
                    }
                });

                ui.add_space(10.0);
            });

            ui.add_space(10.0);
        });
}

// Éditeur de fonctions avec indicateur d'exécution
fn function_editor_ui(
    ui: &mut egui::Ui,
    edit_state: &mut EguiEditState,
    level_manager: &mut LevelManager,
    textures: &InstructionTextures,
    execution_engine: &ExecutionEngine,
) {
    let level_data = level_manager
        .get_current_level()
        .map(|level| (level.id, level.function_limits.clone()))
        .unwrap_or((0, vec![]));

    let (level_id, function_limits) = level_data;

    // Obtenir la position actuelle d'exécution
    let current_function = if execution_engine.is_executing() || execution_engine.is_paused() {
        Some(execution_engine.get_current_function())
    } else {
        None
    };
    let current_instruction = if execution_engine.is_executing() || execution_engine.is_paused() {
        Some(execution_engine.get_current_instruction())
    } else {
        None
    };

    if let Some(problem_state) = level_manager.get_problem_state_mut(level_id) {
        for (func_id, function) in problem_state.functions.iter_mut().enumerate() {
            let limit = function_limits.get(func_id).copied().unwrap_or(10);

            ui.horizontal(|ui| {
                ui.add_space(8.0);

                // Label de la fonction
                ui.label(
                    egui::RichText::new(format!("F{}", func_id + 1))
                        .size(18.0)
                        .strong(),
                );
                ui.add_space(10.0);

                // Slots d'instructions
                for slot_index in 0..limit {
                    let current_instruction_in_slot = if slot_index < function.len() {
                        function[slot_index].clone()
                    } else {
                        Instruction::Noop
                    };

                    // Vérifier si c'est l'instruction en cours d'exécution
                    let is_executing = current_function == Some(func_id)
                        && current_instruction == Some(slot_index);

                    instruction_slot_ui(
                        ui,
                        edit_state,
                        function,
                        slot_index,
                        &current_instruction_in_slot,
                        textures,
                        is_executing,
                    );

                    ui.add_space(2.0);
                }
            });

            ui.add_space(8.0);
        }
    }
}

// Helper pour déterminer quelle texture utiliser selon l'instruction
fn get_texture_for_instruction<'a>(
    instruction: &Instruction,
    textures: &'a InstructionTextures,
) -> &'a Option<egui::TextureHandle> {
    match instruction {
        Instruction::Forward => &textures.forward,
        Instruction::TurnLeft => &textures.turn_left,
        Instruction::TurnRight => &textures.turn_right,
        Instruction::ConditionalRed(inner)
        | Instruction::ConditionalGreen(inner)
        | Instruction::ConditionalBlue(inner) => {
            // Récursion pour obtenir la texture de l'instruction interne
            match inner.as_ref() {
                Instruction::Forward => &textures.forward,
                Instruction::TurnLeft => &textures.turn_left,
                Instruction::TurnRight => &textures.turn_right,
                Instruction::CallFunction(_) | _ => &None,
            }
        }
        Instruction::CallFunction(_) => &None,
        _ => &None,
    }
}

// UI pour un slot d'instruction avec indicateur d'exécution
fn instruction_slot_ui(
    ui: &mut egui::Ui,
    edit_state: &mut EguiEditState,
    function: &mut Vec<Instruction>,
    slot_index: usize,
    current_instruction: &Instruction,
    textures: &InstructionTextures,
    is_executing: bool,
) {
    let (text, color, use_image) = instruction_display_info(current_instruction);
    let slot_size = egui::Vec2::new(38.0, 38.0); // Taille réduite des slots

    // Créer un bouton personnalisé avec image redimensionnée
    let (rect, response) = ui.allocate_exact_size(slot_size, egui::Sense::click());

    // Dessiner le fond du bouton
    let visuals = ui.style().interact(&response);

    // Définir le stroke selon si c'est en cours d'exécution
    let stroke = if is_executing {
        egui::Stroke::new(3.0, egui::Color32::WHITE)
    } else {
        egui::Stroke::new(1.0, egui::Color32::from_gray(100))
    };

    ui.painter().rect(
        rect,
        visuals.corner_radius,
        color,
        stroke,
        egui::StrokeKind::Outside,
    );

    // Dessiner l'image ou le texte
    if use_image && !matches!(current_instruction, Instruction::Noop) {
        let texture = get_texture_for_instruction(current_instruction, textures);

        if let Some(tex) = texture {
            // Dessiner l'image redimensionnée
            let image_size = 25.0; // Taille de l'image dans le slot (réduite)
            let image_rect =
                egui::Rect::from_center_size(rect.center(), egui::Vec2::splat(image_size));
            ui.painter().image(
                tex.id(),
                image_rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        } else {
            // Texte de secours
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::default(),
                egui::Color32::WHITE,
            );
        }
    } else if !text.is_empty() {
        // Dessiner le texte pour les fonctions
        let font_size = if matches!(current_instruction, Instruction::CallFunction(_)) {
            14.0 // Taille réduite
        } else {
            12.0
        };
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(font_size),
            egui::Color32::WHITE,
        );
    }

    if response.clicked() {
        handle_slot_click(edit_state, function, slot_index);
    }

    if !matches!(current_instruction, Instruction::Noop) {
        response.on_hover_text(instruction_to_string(current_instruction));
    }
}

// Bouton d'instruction avec image
fn instruction_image_button(
    ui: &mut egui::Ui,
    edit_state: &mut EguiEditState,
    texture: &Option<egui::TextureHandle>,
    tooltip: &str,
    instruction: Instruction,
) {
    let selected =
        matches!(&edit_state.selected_instruction, Some(inst) if same_variant(inst, &instruction));
    let button_size = egui::Vec2::new(40.0, 40.0); // Taille réduite

    // Créer un bouton personnalisé
    let (rect, response) = ui.allocate_exact_size(button_size, egui::Sense::click());

    // Couleur de fond selon la sélection
    let bg_color = if selected {
        egui::Color32::from_gray(220)
    } else {
        egui::Color32::from_gray(160)
    };

    // Dessiner le fond
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(3),
        bg_color,
        egui::Stroke::new(1.0, egui::Color32::from_gray(100)),
        egui::StrokeKind::Outside,
    );

    // Dessiner l'image ou le texte de secours
    if let Some(tex) = texture {
        let image_size = 28.0; // Taille de l'image dans le bouton (réduite)
        let image_rect = egui::Rect::from_center_size(rect.center(), egui::Vec2::splat(image_size));
        let tint = if selected {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_gray(200)
        };
        ui.painter().image(
            tex.id(),
            image_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            tint,
        );
    } else {
        // Fallback si l'image n'est pas chargée
        let icon = match instruction {
            Instruction::Forward => "→",
            Instruction::TurnLeft => "↶",
            Instruction::TurnRight => "↷",
            _ => "?",
        };
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            icon,
            egui::FontId::proportional(18.0),
            egui::Color32::WHITE,
        );
    }

    if response.on_hover_text(tooltip).clicked() {
        if selected {
            edit_state.selected_instruction = None;
        } else {
            edit_state.selected_instruction = Some(instruction);
            edit_state.selected_condition = None;
        }
    }
}

// Bouton d'instruction avec texte (pour les fonctions)
fn instruction_text_button(
    ui: &mut egui::Ui,
    edit_state: &mut EguiEditState,
    text: &str,
    tooltip: &str,
    instruction: Instruction,
    font_size: f32,
) {
    let selected =
        matches!(&edit_state.selected_instruction, Some(inst) if same_variant(inst, &instruction));
    let color = if selected {
        egui::Color32::from_gray(220)
    } else {
        egui::Color32::from_gray(160)
    };

    let rich_text = egui::RichText::new(text).size(font_size);

    if ui
        .add_sized(
            [40.0, 40.0], // Taille réduite
            egui::Button::new(rich_text).fill(color),
        )
        .on_hover_text(tooltip)
        .clicked()
    {
        if selected {
            edit_state.selected_instruction = None;
        } else {
            edit_state.selected_instruction = Some(instruction);
            edit_state.selected_condition = None;
        }
    }
}

// Fonction helper pour extraire l'instruction interne d'une instruction conditionnelle
fn unwrap_conditional(instruction: &Instruction) -> Instruction {
    match instruction {
        Instruction::ConditionalRed(inner) |
        Instruction::ConditionalGreen(inner) |
        Instruction::ConditionalBlue(inner) => inner.as_ref().clone(),
        other => other.clone(),
    }
}

// Gestion du clic sur un slot
fn handle_slot_click(
    edit_state: &mut EguiEditState,
    function: &mut Vec<Instruction>,
    slot_index: usize,
) {
    let selected_instruction = edit_state.selected_instruction.clone();
    let selected_condition = edit_state.selected_condition.clone();

    match (selected_instruction, selected_condition) {
        // Instruction simple
        (Some(instruction), None) => {
            ensure_function_size(function, slot_index);
            function[slot_index] = instruction;
            edit_state.selected_instruction = None;
        }

        // Condition seule - wrapper l'instruction existante
        (None, Some(condition)) => {
            if slot_index < function.len() && !matches!(function[slot_index], Instruction::Noop) {
                // Extraire l'instruction de base si elle est déjà conditionnelle
                let base_instruction = unwrap_conditional(&function[slot_index]);

                // Wrapper toute instruction non-vide, y compris CallFunction
                if !matches!(base_instruction, Instruction::Noop) {
                    function[slot_index] = wrap_with_condition(base_instruction, condition);
                }
            }
            edit_state.selected_condition = None;
        }

        // Instruction + condition
        (Some(instruction), Some(condition)) => {
            ensure_function_size(function, slot_index);
            function[slot_index] = wrap_with_condition(instruction, condition);
            edit_state.selected_instruction = None;
            edit_state.selected_condition = None;
        }

        // Rien - effacer
        (None, None) => {
            if slot_index < function.len() {
                function[slot_index] = Instruction::Noop;
            }
        }
    }
}

// Fonctions utilitaires
fn condition_button(
    ui: &mut egui::Ui,
    edit_state: &mut EguiEditState,
    label: &str,
    tile_color: TileColor,
    color: egui::Color32,
    size: f32,
) {
    let selected = matches!(&edit_state.selected_condition, Some(cond) if *cond == tile_color);
    let button_color = if selected {
        color.gamma_multiply(1.5)
    } else {
        color
    };

    if ui
        .add_sized([size, size], egui::Button::new(label).fill(button_color))
        .clicked()
    {
        if selected {
            edit_state.selected_condition = None;
        } else {
            edit_state.selected_condition = Some(tile_color);
            edit_state.selected_instruction = None;
        }
    }
}

fn instruction_display_info(instruction: &Instruction) -> (String, egui::Color32, bool) {
    match instruction {
        Instruction::Noop => ("".to_string(), egui::Color32::from_gray(80), false),
        Instruction::Forward => ("→".to_string(), egui::Color32::WHITE, true),
        Instruction::TurnLeft => ("↶".to_string(), egui::Color32::WHITE, true),
        Instruction::TurnRight => ("↷".to_string(), egui::Color32::WHITE, true),
        Instruction::CallFunction(id) => {
            (format!("F{}", id + 1), egui::Color32::from_gray(140), false)
        }
        Instruction::ConditionalRed(inner) => {
            match inner.as_ref() {
                Instruction::CallFunction(id) => {
                    (format!("F{}", id + 1), egui::Color32::from_rgb(200, 80, 80), false)
                }
                _ => ("".to_string(), egui::Color32::from_rgb(200, 80, 80), true)
            }
        }
        Instruction::ConditionalGreen(inner) => {
            match inner.as_ref() {
                Instruction::CallFunction(id) => {
                    (format!("F{}", id + 1), egui::Color32::from_rgb(80, 200, 80), false)
                }
                _ => ("".to_string(), egui::Color32::from_rgb(80, 200, 80), true)
            }
        }
        Instruction::ConditionalBlue(inner) => {
            match inner.as_ref() {
                Instruction::CallFunction(id) => {
                    (format!("F{}", id + 1), egui::Color32::from_rgb(80, 80, 200), false)
                }
                _ => ("".to_string(), egui::Color32::from_rgb(80, 80, 200), true)
            }
        }
    }
}

fn instruction_to_string(instruction: &Instruction) -> String {
    match instruction {
        Instruction::Forward => "Avancer".to_string(),
        Instruction::TurnLeft => "Tourner à gauche".to_string(),
        Instruction::TurnRight => "Tourner à droite".to_string(),
        Instruction::CallFunction(id) => format!("Appeler F{}", id + 1),
        Instruction::ConditionalRed(inner) => format!("Si Rouge: {}", instruction_to_string(inner)),
        Instruction::ConditionalGreen(inner) => {
            format!("Si Vert: {}", instruction_to_string(inner))
        }
        Instruction::ConditionalBlue(inner) => format!("Si Bleu: {}", instruction_to_string(inner)),
        Instruction::Noop => "Vide".to_string(),
    }
}

fn wrap_with_condition(instruction: Instruction, condition: TileColor) -> Instruction {
    match condition {
        TileColor::Red => Instruction::ConditionalRed(Box::new(instruction)),
        TileColor::Green => Instruction::ConditionalGreen(Box::new(instruction)),
        TileColor::Blue => Instruction::ConditionalBlue(Box::new(instruction)),
        TileColor::Gray => instruction,
    }
}

fn ensure_function_size(function: &mut Vec<Instruction>, slot_index: usize) {
    while function.len() <= slot_index {
        function.push(Instruction::Noop);
    }
}

fn same_variant(a: &Instruction, b: &Instruction) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn reset_level_state(
    robot_query: &mut Query<&mut Robot, With<CurrentLevel>>,
    grid_query: &mut Query<&mut Grid, With<CurrentLevel>>,
    level_manager: &mut LevelManager,
) {
    if let Ok(mut robot) = robot_query.single_mut() {
        robot.reset_to_start();
    }

    if let Ok(mut grid) = grid_query.single_mut() {
        // Reset les étoiles seulement si le niveau n'est pas déjà complété
        if let Some(current_level) = level_manager.get_current_level() {
            let is_completed = level_manager
                .get_problem_state(current_level.id)
                .map(|state| state.is_completed)
                .unwrap_or(false);

            if !is_completed {
                // Réinitialiser l'état des étoiles depuis les données du niveau
                for (i, tile_opt) in grid.tiles.iter_mut().enumerate() {
                    if let Some(tile) = tile_opt {
                        tile.star_collected = false;
                    }
                }

                // Réinitialiser le compteur d'étoiles
                if let Some(problem_state) = level_manager.get_problem_state_mut(current_level.id) {
                    problem_state.reset_stars();
                }
            }
        }
    }
}

fn clear_all_instructions(level_manager: &mut LevelManager) {
    if let Some(level) = level_manager.get_current_level() {
        let level_id = level.id;
        if let Some(problem_state) = level_manager.get_problem_state_mut(level_id) {
            for function in &mut problem_state.functions {
                function.clear();
            }
        }
    }
}

// Plugin pour l'UI egui
pub struct EguiUIPlugin;

impl Plugin for EguiUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .init_resource::<EguiEditState>()
        .init_resource::<InstructionTextures>()
        .add_systems(
            EguiContextPass,
            load_instruction_textures
                .run_if(in_state(GameState::Editing).or(in_state(GameState::Tutorial))),
        )
        .add_systems(
            EguiContextPass,
            ui_system.run_if(in_state(GameState::Editing).or(in_state(GameState::Tutorial))),
        );
    }
}
