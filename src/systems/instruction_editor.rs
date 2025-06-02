use crate::components::ui::*;
use crate::resources::level::*;
use crate::structs::controls::*;
use crate::structs::tile::TileColor;
use bevy::prelude::*;

// Resource pour gérer l'état d'édition
#[derive(Resource, Default)]
pub struct InstructionEditState {
    pub selected_instruction: Option<Instruction>,
    pub selected_condition: Option<TileColor>,
    pub selected_slot: Option<(usize, usize)>, // (function_id, slot_index)
}

// Système pour gérer la sélection d'instructions dans la palette
pub fn handle_instruction_palette_system(
    mut interaction_query: Query<(&Interaction, &InstructionButton), (Changed<Interaction>, With<Button>)>,
    mut edit_state: ResMut<InstructionEditState>,
) {
    for (interaction, instruction_btn) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            edit_state.selected_instruction = Some(instruction_btn.instruction.clone());
            edit_state.selected_condition = None; // Reset condition quand on sélectionne une instruction
            info!("Instruction sélectionnée: {:?}", instruction_btn.instruction);
        }
    }
}

// Nouveau système pour gérer les boutons de couleur
pub fn handle_color_condition_system(
    mut interaction_query: Query<(&Interaction, &ColorConditionButton, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut edit_state: ResMut<InstructionEditState>,
) {
    for (interaction, color_btn, mut background) in &mut interaction_query {
        let base_color = color_btn.color;

        // Gestion des couleurs hover/pressed
        match interaction {
            Interaction::Pressed => {
                // Déterminer la couleur de tuile selon la couleur du bouton
                let tile_color = if base_color.to_srgba().red > 0.5 {
                    TileColor::Red
                } else if base_color.to_srgba().green > 0.5 {
                    TileColor::Green
                } else {
                    TileColor::Blue
                };

                edit_state.selected_condition = Some(tile_color);
                edit_state.selected_instruction = None; // Reset instruction quand on sélectionne une condition
                *background = BackgroundColor(Color::srgb(0.9, 0.9, 0.9)); // Couleur pressée
            }
            Interaction::Hovered => {
                // Éclaircir la couleur en hover
                let lighter = Color::srgb(
                    (base_color.to_srgba().red + 0.2).min(1.0),
                    (base_color.to_srgba().green + 0.2).min(1.0),
                    (base_color.to_srgba().blue + 0.2).min(1.0),
                );
                *background = BackgroundColor(lighter);
            }
            Interaction::None => {
                *background = BackgroundColor(base_color);
            }
        }
    }
}

// Système pour gérer les clics sur les slots d'instructions
pub fn handle_instruction_slot_system(
    mut interaction_query: Query<(&Interaction, &InstructionSlot, Entity, &mut BackgroundColor, &mut SlotConditionColor), (Changed<Interaction>, With<Button>)>,
    mut edit_state: ResMut<InstructionEditState>,
    mut level_manager: ResMut<LevelManager>,
    mut text_query: Query<&mut Text>,
    children_query: Query<&Children>,
) {
    for (interaction, slot, entity, mut background, mut slot_color) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Récupérer l'instruction actuelle dans ce slot
            let current_instruction = get_instruction_at_slot(&level_manager, slot.function_id, slot.slot_index);

            // Vérifier ce qui est sélectionné
            match (&edit_state.selected_instruction, &edit_state.selected_condition) {
                // Instruction simple sélectionnée
                (Some(instruction), None) => {
                    // Si la case a une couleur de condition, créer une instruction conditionnelle
                    if let Some(condition_color) = &slot_color.color {
                        let tile_color = color_to_tile_color(*condition_color);
                        let conditional_instruction = create_conditional_instruction(instruction.clone(), tile_color);
                        place_instruction_in_slot(
                            &mut level_manager,
                            &mut text_query,
                            &children_query,
                            slot.function_id,
                            slot.slot_index,
                            conditional_instruction,
                            entity,
                        );
                        info!("Instruction placée avec condition existante");
                    } else {
                        // Case normale, placer l'instruction directement
                        place_instruction_in_slot(
                            &mut level_manager,
                            &mut text_query,
                            &children_query,
                            slot.function_id,
                            slot.slot_index,
                            instruction.clone(),
                            entity,
                        );
                    }
                    edit_state.selected_instruction = None;
                }
                // Condition sélectionnée - colorer et/ou wrapper
                (None, Some(condition)) => {
                    let color = tile_color_to_color(*condition);

                    // Colorer visuellement la case
                    *background = BackgroundColor(color);
                    slot_color.color = Some(color);

                    // Si il y a une instruction existante (pas Noop), la wrapper
                    if let Some(existing) = current_instruction {
                        if !matches!(existing, Instruction::Noop) {
                            let wrapped = create_conditional_instruction(existing, *condition);
                            place_instruction_in_slot(
                                &mut level_manager,
                                &mut text_query,
                                &children_query,
                                slot.function_id,
                                slot.slot_index,
                                wrapped,
                                entity,
                            );
                            info!("Instruction existante wrappée avec couleur");
                        } else {
                            info!("Case vide colorée - prête pour instruction");
                        }
                    } else {
                        info!("Case hors fonction colorée");
                    }
                    edit_state.selected_condition = None;
                }
                // Instruction ET condition sélectionnées - créer instruction conditionnelle colorée
                (Some(instruction), Some(condition)) => {
                    let conditional_instruction = create_conditional_instruction(instruction.clone(), *condition);
                    let color = tile_color_to_color(*condition);

                    // Colorer visuellement la case
                    *background = BackgroundColor(color);
                    slot_color.color = Some(color);

                    place_instruction_in_slot(
                        &mut level_manager,
                        &mut text_query,
                        &children_query,
                        slot.function_id,
                        slot.slot_index,
                        conditional_instruction,
                        entity,
                    );
                    edit_state.selected_instruction = None;
                    edit_state.selected_condition = None;
                }
                // Rien de sélectionné - supprimer l'instruction et la couleur
                (None, None) => {
                    // Restaurer couleur par défaut
                    *background = BackgroundColor(Color::srgb(0.6, 0.6, 0.6));
                    slot_color.color = None;

                    remove_instruction_from_slot(
                        &mut level_manager,
                        &mut text_query,
                        &children_query,
                        slot.function_id,
                        slot.slot_index,
                        entity,
                    );
                }
            }
        }
    }
}

// Fonction helper pour récupérer l'instruction à un slot donné
fn get_instruction_at_slot(level_manager: &LevelManager, function_id: usize, slot_index: usize) -> Option<Instruction> {
    let level_id = level_manager.get_current_level()?.id;
    let problem_state = level_manager.get_problem_state(level_id)?;

    if function_id < problem_state.functions.len() {
        let function = &problem_state.functions[function_id];
        if slot_index < function.len() {
            Some(function[slot_index].clone())
        } else {
            Some(Instruction::Noop)
        }
    } else {
        None
    }
}

// Fonction helper pour convertir TileColor vers Color
fn tile_color_to_color(tile_color: TileColor) -> Color {
    match tile_color {
        TileColor::Red => Color::srgb(0.9, 0.4, 0.4),
        TileColor::Green => Color::srgb(0.4, 0.9, 0.4),
        TileColor::Blue => Color::srgb(0.4, 0.4, 0.9),
        TileColor::Gray => Color::srgb(0.6, 0.6, 0.6),
    }
}

// Fonction helper pour convertir Color vers TileColor
fn color_to_tile_color(color: Color) -> TileColor {
    let rgba = color.to_srgba();
    if rgba.red > 0.7 {
        TileColor::Red
    } else if rgba.green > 0.7 {
        TileColor::Green
    } else if rgba.blue > 0.7 {
        TileColor::Blue
    } else {
        TileColor::Gray
    }
}

// Fonction pour créer une instruction conditionnelle
fn create_conditional_instruction(instruction: Instruction, condition: TileColor) -> Instruction {
    match condition {
        TileColor::Red => Instruction::ConditionalRed(Box::new(instruction)),
        TileColor::Green => Instruction::ConditionalGreen(Box::new(instruction)),
        TileColor::Blue => Instruction::ConditionalBlue(Box::new(instruction)),
        TileColor::Gray => instruction, // Pas de condition pour gris
    }
}

// Fonction pour wrapper une instruction existante avec une condition
fn wrap_instruction_with_condition(
    level_manager: &mut ResMut<LevelManager>,
    text_query: &mut Query<&mut Text>,
    children_query: &Query<&Children>,
    function_id: usize,
    slot_index: usize,
    condition: TileColor,
    slot_entity: Entity,
) {
    let level_id = level_manager.get_current_level().map(|level| level.id);
    if let Some(level_id) = level_id {
        if let Some(problem_state) = level_manager.get_problem_state_mut(level_id) {
            if function_id < problem_state.functions.len() {
                let function = &mut problem_state.functions[function_id];

                // Vérifier si l'instruction existe sans étendre automatiquement
                if slot_index < function.len() {
                    let existing_instruction = function[slot_index].clone();

                    match existing_instruction {
                        Instruction::Noop => {
                            // Case vide - juste colorer la case, ne pas modifier l'instruction
                            // L'instruction reste Noop, mais la case est colorée
                        }
                        Instruction::ConditionalRed(_) |
                        Instruction::ConditionalGreen(_) |
                        Instruction::ConditionalBlue(_) => {
                            info!("Instruction déjà conditionnelle - pas de changement");
                        }
                        _ => {
                            // Instruction existante - la wrapper
                            let wrapped = create_conditional_instruction(existing_instruction, condition);
                            function[slot_index] = wrapped.clone();
                            update_slot_display(text_query, children_query, slot_entity, &wrapped);
                        }
                    }
                } else {
                }
            }
        }
    }
}

// Fonction pour placer une instruction dans un slot
fn place_instruction_in_slot(
    level_manager: &mut ResMut<LevelManager>,
    text_query: &mut Query<&mut Text>,
    children_query: &Query<&Children>,
    function_id: usize,
    slot_index: usize,
    instruction: Instruction,
    slot_entity: Entity,
) {
    // Met à jour les données du niveau
    let level_id = level_manager.get_current_level().map(|level| level.id);
    if let Some(level_id) = level_id {
        if let Some(problem_state) = level_manager.get_problem_state_mut(level_id) {
            if function_id < problem_state.functions.len() {
                let function = &mut problem_state.functions[function_id];

                // Étend la fonction si nécessaire
                while function.len() <= slot_index {
                    function.push(Instruction::Noop);
                }

                // Place l'instruction
                function[slot_index] = instruction.clone();

                // Met à jour l'affichage du slot
                update_slot_display(text_query, children_query, slot_entity, &instruction);

                info!("Instruction {:?} placée dans F{} slot {}", instruction, function_id + 1, slot_index);
            }
        }
    }
}

// Fonction pour supprimer une instruction d'un slot
fn remove_instruction_from_slot(
    level_manager: &mut ResMut<LevelManager>,
    text_query: &mut Query<&mut Text>,
    children_query: &Query<&Children>,
    function_id: usize,
    slot_index: usize,
    slot_entity: Entity,
) {
    // Met à jour les données du niveau
    let level_id = level_manager.get_current_level().map(|level| level.id);
    if let Some(level_id) = level_id {
        if let Some(problem_state) = level_manager.get_problem_state_mut(level_id) {
            if function_id < problem_state.functions.len() {
                let function = &mut problem_state.functions[function_id];

                if slot_index < function.len() {
                    function[slot_index] = Instruction::Noop;

                    // Met à jour l'affichage du slot
                    update_slot_display(text_query, children_query, slot_entity, &Instruction::Noop);

                    info!("Instruction supprimée de F{} slot {}", function_id + 1, slot_index);
                }
            }
        }
    }
}

// Fonction pour mettre à jour l'affichage d'un slot
fn update_slot_display(
    text_query: &mut Query<&mut Text>,
    children_query: &Query<&Children>,
    slot_entity: Entity,
    instruction: &Instruction,
) {
    if let Ok(children) = children_query.get(slot_entity) {
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                **text = instruction_to_display_string(instruction);
            }
        }
    }
}

// Fonction pour convertir une instruction en texte d'affichage
fn instruction_to_display_string(instruction: &Instruction) -> String {
    match instruction {
        Instruction::Forward => "→".to_string(),
        Instruction::TurnLeft => "⤺".to_string(),
        Instruction::TurnRight => "⤻".to_string(),
        Instruction::CallFunction(id) => format!("F{}", id + 1),
        Instruction::ConditionalRed(_) => "R?".to_string(),
        Instruction::ConditionalGreen(_) => "G?".to_string(),
        Instruction::ConditionalBlue(_) => "B?".to_string(),
        Instruction::Noop => "".to_string(),
    }
}

// Système pour mettre à jour l'affichage des fonctions selon l'état actuel
pub fn update_function_display_system(
    level_manager: Res<LevelManager>,
    mut text_query: Query<&mut Text>,
    mut slot_query: Query<(&InstructionSlot, Entity, &mut BackgroundColor, &mut SlotConditionColor), With<InstructionSlot>>,
    children_query: Query<&Children>,
) {
    if !level_manager.is_changed() {
        return;
    }

    let Some(current_level) = level_manager.get_current_level() else {
        return;
    };

    let Some(problem_state) = level_manager.get_problem_state(current_level.id) else {
        return;
    };

    // Met à jour tous les slots d'instructions
    for (slot, entity, mut background, mut slot_color) in slot_query.iter_mut() {
        if slot.function_id < problem_state.functions.len() {
            let function = &problem_state.functions[slot.function_id];
            let instruction = if slot.slot_index < function.len() {
                &function[slot.slot_index]
            } else {
                &Instruction::Noop
            };

            // Met à jour le texte
            update_slot_display(&mut text_query, &children_query, entity, instruction);

            // Met à jour la couleur selon l'instruction
            match instruction {
                Instruction::ConditionalRed(_) => {
                    let color = Color::srgb(0.9, 0.4, 0.4);
                    *background = BackgroundColor(color);
                    slot_color.color = Some(color);
                }
                Instruction::ConditionalGreen(_) => {
                    let color = Color::srgb(0.4, 0.9, 0.4);
                    *background = BackgroundColor(color);
                    slot_color.color = Some(color);
                }
                Instruction::ConditionalBlue(_) => {
                    let color = Color::srgb(0.4, 0.4, 0.9);
                    *background = BackgroundColor(color);
                    slot_color.color = Some(color);
                }
                Instruction::Noop => {
                    // Garder la couleur existante si elle existe, sinon gris par défaut
                    if slot_color.color.is_none() {
                        *background = BackgroundColor(Color::srgb(0.6, 0.6, 0.6));
                    }
                }
                _ => {
                    // Instruction normale - garder couleur de condition si elle existe
                    if slot_color.color.is_none() {
                        *background = BackgroundColor(Color::srgb(0.6, 0.6, 0.6));
                    }
                }
            }
        }
    }
}

// Système pour gérer les raccourcis clavier
pub fn handle_keyboard_shortcuts_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut edit_state: ResMut<InstructionEditState>,
) {
    // Raccourcis pour sélectionner rapidement des instructions
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        edit_state.selected_instruction = Some(Instruction::Forward);
        edit_state.selected_condition = None;
    } else if keyboard.just_pressed(KeyCode::ArrowLeft) {
        edit_state.selected_instruction = Some(Instruction::TurnLeft);
        edit_state.selected_condition = None;
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        edit_state.selected_instruction = Some(Instruction::TurnRight);
        edit_state.selected_condition = None;
    } else if keyboard.just_pressed(KeyCode::Digit1) {
        edit_state.selected_instruction = Some(Instruction::CallFunction(0));
        edit_state.selected_condition = None;
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        edit_state.selected_instruction = Some(Instruction::CallFunction(1));
        edit_state.selected_condition = None;
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        edit_state.selected_instruction = Some(Instruction::CallFunction(2));
        edit_state.selected_condition = None;
    } else if keyboard.just_pressed(KeyCode::KeyR) {
        edit_state.selected_condition = Some(TileColor::Red);
        edit_state.selected_instruction = None;
    } else if keyboard.just_pressed(KeyCode::KeyG) {
        edit_state.selected_condition = Some(TileColor::Green);
        edit_state.selected_instruction = None;
    } else if keyboard.just_pressed(KeyCode::KeyB) {
        edit_state.selected_condition = Some(TileColor::Blue);
        edit_state.selected_instruction = None;
    } else if keyboard.just_pressed(KeyCode::Delete) {
        edit_state.selected_instruction = None;
        edit_state.selected_condition = None;
    }
}

// Système pour afficher l'instruction sélectionnée
pub fn display_selected_instruction_system(
    edit_state: Res<InstructionEditState>,
    mut commands: Commands,
    selected_display_query: Query<Entity, With<StatusMessage>>,
) {
    // Nettoie l'affichage précédent
    for entity in selected_display_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Affiche l'instruction sélectionnée
    if let Some(instruction) = &edit_state.selected_instruction {
        let text = format!("Instruction sélectionnée: {}", instruction_to_display_string(instruction));

        commands.spawn((
            Text::new(text),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                bottom: Val::Px(200.0),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.8, 0.8)),
            StatusMessage,
        ));
    }
}