// systems/ui_egui.rs
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiContextPass};
use crate::components::ui::*;
use crate::resources::level::*;
use crate::resources::execution::*;
use crate::structs::controls::*;
use crate::structs::tile::TileColor;
use crate::states::game::GameState;

// Resource pour l'état d'édition avec egui
#[derive(Resource, Default)]
pub struct EguiEditState {
    pub selected_instruction: Option<Instruction>,
    pub selected_condition: Option<TileColor>,
}

// Système principal d'interface egui
pub fn ui_system(
    mut contexts: EguiContexts,
    mut edit_state: ResMut<EguiEditState>,
    mut level_manager: ResMut<LevelManager>,
    mut execution_engine: ResMut<ExecutionEngine>,
    mut robot_query: Query<&mut crate::components::robot::Robot, With<crate::components::level::CurrentLevel>>,
    mut grid_query: Query<&mut crate::components::grid::Grid, With<crate::components::level::CurrentLevel>>,
) {
    let ctx = contexts.ctx_mut();

    // Panel principal
    egui::Window::new("Robozzle Interface")
        .resizable(true)
        .default_size([800.0, 600.0])
        .show(ctx, |ui| {

            // Boutons de contrôle
            ui.horizontal(|ui| {
                control_buttons_ui(ui, &mut execution_engine, &mut robot_query, &mut grid_query, &mut level_manager);
            });

            ui.separator();

            // Récupérer les données du niveau une seule fois
            let level_data = level_manager.get_current_level().map(|level| (level.id, level.function_limits.len()));

            if let Some((level_id, function_count)) = level_data {
                // Palette d'instructions
                instruction_palette_ui(ui, &mut edit_state, function_count);

                ui.separator();

                // Éditeur de fonctions
                function_editor_ui(ui, &mut edit_state, &mut level_manager, level_id as u32);
            }

            ui.separator();

            // État de sélection
            selection_state_ui(ui, &edit_state);

            // Messages d'erreur
            if let Some(error) = execution_engine.get_error() {
                ui.colored_label(egui::Color32::RED, format!("Erreur: {}", error));
            }
        });
}

// Interface des boutons de contrôle
fn control_buttons_ui(
    ui: &mut egui::Ui,
    execution_engine: &mut ExecutionEngine,
    robot_query: &mut Query<&mut crate::components::robot::Robot, With<crate::components::level::CurrentLevel>>,
    grid_query: &mut Query<&mut crate::components::grid::Grid, With<crate::components::level::CurrentLevel>>,
    level_manager: &mut LevelManager,
) {
    // Bouton Lancer/Pause
    let button_text = if execution_engine.is_stopped() {
        "🚀 Lancer"
    } else if execution_engine.is_paused() {
        "▶️ Reprendre"
    } else {
        "⏸️ Pause"
    };

    let button_color = if execution_engine.is_executing() {
        egui::Color32::from_rgb(200, 200, 50) // Jaune
    } else {
        egui::Color32::from_rgb(50, 200, 50) // Vert
    };

    if ui.add(egui::Button::new(button_text).fill(button_color)).clicked() {
        if execution_engine.is_stopped() {
            // Reset robot avant lancement
            if let Ok(mut robot) = robot_query.single_mut() {
                robot.reset_to_start();
            }
            // Reset étoiles
            if let Ok(mut grid) = grid_query.single_mut() {
                for tile_opt in &mut grid.tiles {
                    if let Some(tile) = tile_opt {
                        tile.star_collected = false;
                    }
                }
            }
            execution_engine.start_execution();
        } else if execution_engine.is_executing() {
            execution_engine.pause();
        } else if execution_engine.is_paused() {
            execution_engine.resume();
        }
    }

    // Bouton vitesse
    let speed_text = format!("⚡ x{:?}", execution_engine.get_speed());
    if ui.add(egui::Button::new(speed_text).fill(egui::Color32::from_rgb(200, 200, 50))).clicked() {
        execution_engine.change_speed();
    }

    // Bouton Reset
    if ui.add(egui::Button::new("🔄 Reset").fill(egui::Color32::from_rgb(200, 150, 50))).clicked() {
        execution_engine.stop();
        execution_engine.clear_error();
        if let Ok(mut robot) = robot_query.single_mut() {
            robot.reset_to_start();
        }
        if let Ok(mut grid) = grid_query.single_mut() {
            for tile_opt in &mut grid.tiles {
                if let Some(tile) = tile_opt {
                    tile.star_collected = false;
                }
            }
        }
    }

    // Bouton Clear
    if ui.add(egui::Button::new("🗑️ Clear").fill(egui::Color32::from_rgb(200, 50, 50))).clicked() {
        execution_engine.stop();
        // Vider toutes les fonctions
        if let Some(level) = level_manager.get_current_level() {
            let level_id = level.id;
            if let Some(problem_state) = level_manager.get_problem_state_mut(level_id) {
                for function in &mut problem_state.functions {
                    function.clear();
                }
            }
        }
    }
}

// Interface de la palette d'instructions
fn instruction_palette_ui(
    ui: &mut egui::Ui,
    edit_state: &mut EguiEditState,
    function_count: usize,
) {
    ui.label("📋 Instructions:");

    ui.horizontal(|ui| {
        // Instructions de base
        let basic_instructions = vec![
            ("→", Instruction::Forward),
            ("⤺", Instruction::TurnLeft),
            ("⤻", Instruction::TurnRight),
        ];

        for (label, instruction) in basic_instructions {
            let selected = matches!(&edit_state.selected_instruction, Some(inst) if std::mem::discriminant(inst) == std::mem::discriminant(&instruction));
            let color = if selected {
                egui::Color32::from_rgb(100, 150, 255)
            } else {
                egui::Color32::from_rgb(100, 100, 150)
            };

            if ui.add(egui::Button::new(label).fill(color)).clicked() {
                edit_state.selected_instruction = Some(instruction);
                edit_state.selected_condition = None;
            }
        }

        ui.separator();

        // Fonctions dynamiques selon le niveau
        for i in 0..function_count {
            let instruction = Instruction::CallFunction(i);
            let label = format!("F{}", i + 1);
            let selected = matches!(&edit_state.selected_instruction, Some(inst) if matches!(inst, Instruction::CallFunction(id) if *id == i));
            let color = if selected {
                egui::Color32::from_rgb(100, 150, 255)
            } else {
                egui::Color32::from_rgb(100, 100, 150)
            };

            if ui.add(egui::Button::new(label).fill(color)).clicked() {
                edit_state.selected_instruction = Some(instruction);
                edit_state.selected_condition = None;
            }
        }

        ui.separator();

        // Boutons de couleur
        ui.label("🎨");

        let color_conditions = vec![
            (TileColor::Red, egui::Color32::from_rgb(230, 100, 100)),
            (TileColor::Green, egui::Color32::from_rgb(100, 230, 100)),
            (TileColor::Blue, egui::Color32::from_rgb(100, 100, 230)),
        ];

        for (tile_color, color) in color_conditions {
            let selected = matches!(&edit_state.selected_condition, Some(cond) if *cond == tile_color);
            let final_color = if selected {
                egui::Color32::WHITE
            } else {
                color
            };

            if ui.add(egui::Button::new("  ").fill(final_color).min_size(egui::Vec2::new(30.0, 25.0))).clicked() {
                edit_state.selected_condition = Some(tile_color);
                edit_state.selected_instruction = None;
            }
        }

        // Bouton clear sélection
        if ui.add(egui::Button::new("❌").fill(egui::Color32::GRAY)).clicked() {
            edit_state.selected_instruction = None;
            edit_state.selected_condition = None;
        }
    });
}

// Interface de l'éditeur de fonctions
fn function_editor_ui(
    ui: &mut egui::Ui,
    edit_state: &mut EguiEditState,
    level_manager: &mut LevelManager,
    level_id: u32,
) {
    ui.label("🔧 Fonctions:");

    // Récupérer le problem_state une seule fois
    if let Some(problem_state) = level_manager.get_problem_state_mut(level_id as usize) {
        for (func_id, function) in problem_state.functions.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("F{}:", func_id + 1));

                // Créer une grille de slots pour cette fonction
                let max_slots = 8; // Ou selon le niveau
                for slot_index in 0..max_slots {
                    // Cloner l'instruction pour éviter le conflit de borrow
                    let current_instruction = if slot_index < function.len() {
                        function[slot_index].clone()
                    } else {
                        Instruction::Noop
                    };

                    instruction_slot_ui(ui, edit_state, function, func_id, slot_index, &current_instruction);
                }
            });
        }
    }
}

// Interface d'un slot d'instruction individuel
fn instruction_slot_ui(
    ui: &mut egui::Ui,
    edit_state: &mut EguiEditState,
    function: &mut Vec<Instruction>,
    func_id: usize,
    slot_index: usize,
    current_instruction: &Instruction,
) {
    // Déterminer couleur et texte
    let (text, bg_color) = match current_instruction {
        Instruction::Noop => ("".to_string(), egui::Color32::from_rgb(150, 150, 150)),
        Instruction::Forward => ("→".to_string(), egui::Color32::from_rgb(150, 150, 150)),
        Instruction::TurnLeft => ("⤺".to_string(), egui::Color32::from_rgb(150, 150, 150)),
        Instruction::TurnRight => ("⤻".to_string(), egui::Color32::from_rgb(150, 150, 150)),
        Instruction::CallFunction(id) => (format!("F{}", id + 1), egui::Color32::from_rgb(150, 150, 150)),
        Instruction::ConditionalRed(inner) => {
            let inner_text = instruction_to_short_string(inner);
            (format!("R:{}", inner_text), egui::Color32::from_rgb(230, 120, 120))
        },
        Instruction::ConditionalGreen(inner) => {
            let inner_text = instruction_to_short_string(inner);
            (format!("G:{}", inner_text), egui::Color32::from_rgb(120, 230, 120))
        },
        Instruction::ConditionalBlue(inner) => {
            let inner_text = instruction_to_short_string(inner);
            (format!("B:{}", inner_text), egui::Color32::from_rgb(120, 120, 230))
        },
    };

    // Bouton du slot
    let button = egui::Button::new(&text)
        .fill(bg_color)
        .min_size(egui::Vec2::new(60.0, 25.0));

    if ui.add(button).clicked() {
        handle_slot_click(edit_state, function, slot_index);
    }
}

// Gestion du clic sur un slot - CORRIGÉ pour éviter suppression d'instructions et conflit de borrow
fn handle_slot_click(
    edit_state: &mut EguiEditState,
    function: &mut Vec<Instruction>,
    slot_index: usize,
) {
    // Cloner les valeurs sélectionnées pour éviter les conflits de borrow
    let selected_instruction = edit_state.selected_instruction.clone();
    let selected_condition = edit_state.selected_condition.clone();

    match (&selected_instruction, &selected_condition) {
        // Instruction simple sélectionnée
        (Some(instruction), None) => {
            // Étendre la fonction si nécessaire
            while function.len() <= slot_index {
                function.push(Instruction::Noop);
            }
            function[slot_index] = instruction.clone();
            edit_state.selected_instruction = None;
            info!("Instruction placée: {} dans slot {}", instruction_to_display_string(instruction), slot_index);
        },

        // Condition sélectionnée - wrapper l'instruction existante SANS la supprimer
        (None, Some(condition)) => {
            if slot_index < function.len() {
                let existing = function[slot_index].clone();
                if !matches!(existing, Instruction::Noop) {
                    let wrapped = create_conditional_instruction(existing.clone(), *condition);
                    function[slot_index] = wrapped;
                    info!("Instruction {} wrappée avec condition {:?}", instruction_to_display_string(&existing), condition);
                } else {
                    info!("Case vide - sélectionnez une instruction à ajouter");
                }
            } else {
                info!("Slot hors fonction - sélectionnez une instruction à ajouter");
            }
            edit_state.selected_condition = None;
        },

        // Instruction ET condition - créer conditionnelle directement
        (Some(instruction), Some(condition)) => {
            while function.len() <= slot_index {
                function.push(Instruction::Noop);
            }
            let conditional = create_conditional_instruction(instruction.clone(), *condition);
            function[slot_index] = conditional.clone();
            edit_state.selected_instruction = None;
            edit_state.selected_condition = None;
            info!("Instruction conditionnelle créée: {}", instruction_to_display_string(&conditional));
        },

        // Rien de sélectionné - supprimer
        (None, None) => {
            if slot_index < function.len() {
                let old_instruction = function[slot_index].clone();
                function[slot_index] = Instruction::Noop;
                info!("Instruction supprimée: {}", instruction_to_display_string(&old_instruction));
            }
        },
    }
}

// Interface d'état de sélection
fn selection_state_ui(ui: &mut egui::Ui, edit_state: &EguiEditState) {
    ui.horizontal(|ui| {
        ui.label("État:");

        match (&edit_state.selected_instruction, &edit_state.selected_condition) {
            (Some(inst), None) => {
                ui.colored_label(egui::Color32::BLUE, format!("Instruction: {}", instruction_to_display_string(inst)));
            },
            (None, Some(cond)) => {
                let cond_name = match cond {
                    TileColor::Red => "Rouge",
                    TileColor::Green => "Vert",
                    TileColor::Blue => "Bleu",
                    TileColor::Gray => "Gris",
                };
                ui.colored_label(egui::Color32::GREEN, format!("Condition: {}", cond_name));
            },
            (Some(inst), Some(cond)) => {
                let cond_name = match cond {
                    TileColor::Red => "Rouge",
                    TileColor::Green => "Vert",
                    TileColor::Blue => "Bleu",
                    TileColor::Gray => "Gris",
                };
                ui.colored_label(egui::Color32::YELLOW, format!("Instruction: {} + Condition: {}", instruction_to_display_string(inst), cond_name));
            },
            (None, None) => {
                ui.colored_label(egui::Color32::GRAY, "Mode suppression");
            },
        }
    });
}

// Fonctions helper
fn instruction_to_display_string(instruction: &Instruction) -> String {
    match instruction {
        Instruction::Forward => "Avancer".to_string(),
        Instruction::TurnLeft => "Gauche".to_string(),
        Instruction::TurnRight => "Droite".to_string(),
        Instruction::CallFunction(id) => format!("F{}", id + 1),
        Instruction::ConditionalRed(inner) => format!("R:{}", instruction_to_display_string(inner)),
        Instruction::ConditionalGreen(inner) => format!("G:{}", instruction_to_display_string(inner)),
        Instruction::ConditionalBlue(inner) => format!("B:{}", instruction_to_display_string(inner)),
        Instruction::Noop => "".to_string(),
    }
}

fn instruction_to_short_string(instruction: &Instruction) -> String {
    match instruction {
        Instruction::Forward => "→".to_string(),
        Instruction::TurnLeft => "⤺".to_string(),
        Instruction::TurnRight => "⤻".to_string(),
        Instruction::CallFunction(id) => format!("F{}", id + 1),
        Instruction::Noop => "".to_string(),
        _ => "?".to_string(),
    }
}

fn create_conditional_instruction(instruction: Instruction, condition: TileColor) -> Instruction {
    match condition {
        TileColor::Red => Instruction::ConditionalRed(Box::new(instruction)),
        TileColor::Green => Instruction::ConditionalGreen(Box::new(instruction)),
        TileColor::Blue => Instruction::ConditionalBlue(Box::new(instruction)),
        TileColor::Gray => instruction,
    }
}

// Plugin pour remplacer l'UI Bevy native
pub struct EguiUIPlugin;

impl Plugin for EguiUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
            .init_resource::<EguiEditState>()
            .add_systems(EguiContextPass, ui_system.run_if(in_state(GameState::Editing)));
    }
}