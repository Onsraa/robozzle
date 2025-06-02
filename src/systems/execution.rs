use crate::components::grid::*;
use crate::components::level::*;
use crate::components::robot::*;
use crate::resources::execution::*;
use crate::resources::level::*;
use crate::structs::controls::*;
use crate::structs::tile::TileColor;
use bevy::prelude::*;

// Système principal d'exécution des instructions
pub fn execution_system(
    time: Res<Time>,
    mut execution_engine: ResMut<ExecutionEngine>,
    mut robot_query: Query<&mut Robot, With<CurrentLevel>>,
    mut grid_query: Query<&mut Grid, With<CurrentLevel>>,
    level_manager: Res<LevelManager>,
) {
    // Vérifie si il est temps d'exécuter la prochaine instruction
    if !execution_engine.tick(time.delta()) {
        return;
    }

    let Ok(mut robot) = robot_query.single_mut() else {
        return;
    };

    let Ok(mut grid) = grid_query.single_mut() else {
        return;
    };

    let Some(current_level) = level_manager.get_current_level() else {
        return;
    };

    // Obtient l'état du problème pour accéder aux fonctions
    let Some(problem_state) = level_manager.get_problem_state(current_level.id) else {
        return;
    };

    // Obtient l'instruction actuelle
    let current_function = execution_engine.get_current_function();
    let current_instruction_index = execution_engine.get_current_instruction();

    if current_function >= problem_state.functions.len() {
        execution_engine.stop();
        return;
    }

    let function = &problem_state.functions[current_function];

    if current_instruction_index >= function.len() {
        // Fin de fonction - retour ou arrêt
        if !execution_engine.return_from_function() {
            // Plus rien à exécuter
            return;
        }
        // Continue avec l'instruction suivante après le retour
        return;
    }

    let instruction = function[current_instruction_index].clone();

    // Exécute l'instruction
    match execute_instruction(instruction, &mut robot, &mut grid, &mut execution_engine, &level_manager) {
        Ok(()) => {
            // Instruction exécutée avec succès
            execution_engine.advance_instruction();
        }
        Err(error_msg) => {
            // Erreur d'exécution
            execution_engine.set_error(error_msg);
            robot.reset_to_start();
        }
    }
}

// Fonction qui exécute une instruction spécifique
fn  execute_instruction(
    instruction: Instruction,
    robot: &mut Robot,
    grid: &mut Grid,
    execution_engine: &mut ExecutionEngine,
    level_manager: &LevelManager,
) -> Result<(), String> {
    match instruction {
        Instruction::Forward => {
            // Calcule la nouvelle position
            let (dx, dy) = robot.direction.get_offset();
            let new_x = robot.x + dx;
            let new_y = robot.y + dy;

            // Vérifie si la nouvelle position est valide
            if !grid.is_valid_position(new_x, new_y) {
                return Err("Vous êtes en dehors du puzzle".to_string());
            }

            // Déplace le robot
            robot.x = new_x;
            robot.y = new_y;

            // Vérifie si il y a une étoile à collecter
            if let Some(tile) = grid.get_tile_at_mut(new_x, new_y) {
                if tile.has_star && !tile.star_collected {
                    tile.star_collected = true;
                    info!("Étoile collectée à ({}, {})", new_x, new_y);
                }
            }

            Ok(())
        }

        Instruction::TurnLeft => {
            robot.turn_left();
            Ok(())
        }

        Instruction::TurnRight => {
            robot.turn_right();
            Ok(())
        }

        Instruction::CallFunction(function_id) => {
            // Vérifier que la fonction existe
            let level_id = level_manager.get_current_level().map(|level| level.id);
            if let Some(level_id) = level_id {
                if let Some(problem_state) = level_manager.get_problem_state(level_id) {
                    if function_id < problem_state.functions.len() {
                        info!("Appel de fonction {} depuis fonction {} instruction {}", 
                              function_id, execution_engine.get_current_function(), execution_engine.get_current_instruction());
                        execution_engine.call_function(function_id);
                        Ok(())
                    } else {
                        Err(format!("Fonction {} n'existe pas", function_id))
                    }
                } else {
                    Err("Aucun état de problème trouvé".to_string())
                }
            } else {
                Err("Aucun niveau actuel trouvé".to_string())
            }
        }

        Instruction::ConditionalRed(inner_instruction) => {
            if let Some(tile) = grid.get_tile_at(robot.x, robot.y) {
                if tile.color == TileColor::Red {
                    execute_instruction(*inner_instruction, robot, grid, execution_engine, level_manager)?;
                }
            }
            Ok(())
        }

        Instruction::ConditionalGreen(inner_instruction) => {
            if let Some(tile) = grid.get_tile_at(robot.x, robot.y) {
                if tile.color == TileColor::Green {
                    execute_instruction(*inner_instruction, robot, grid, execution_engine, level_manager)?;
                }
            }
            Ok(())
        }

        Instruction::ConditionalBlue(inner_instruction) => {
            if let Some(tile) = grid.get_tile_at(robot.x, robot.y) {
                if tile.color == TileColor::Blue {
                    execute_instruction(*inner_instruction, robot, grid, execution_engine, level_manager)?;
                }
            }
            Ok(())
        }

        Instruction::Noop => {
            // Ne fait rien
            Ok(())
        }
    }
}

// Système pour vérifier si le puzzle est résolu
pub fn check_completion_system(
    grid_query: Query<&Grid, With<CurrentLevel>>,
    mut level_manager: ResMut<LevelManager>,
    execution_engine: Res<ExecutionEngine>,
) {
    let Ok(grid) = grid_query.single() else {
        return;
    };

    // Ne vérifie que si on n'est pas en cours d'exécution
    if execution_engine.is_executing() {
        return;
    }

    let Some(current_level) = level_manager.get_current_level() else {
        return;
    };

    let level_id = current_level.id;
    let total_stars = current_level.total_stars;

    // Compte les étoiles collectées
    let stars_collected = grid.tiles.iter()
        .filter_map(|tile_opt| tile_opt.as_ref())
        .filter(|tile| tile.has_star && tile.star_collected)
        .count();

    // Met à jour l'état du problème
    if let Some(problem_state) = level_manager.get_problem_state_mut(level_id) {
        problem_state.stars_collected = stars_collected;
        problem_state.check_completion(total_stars);

        if problem_state.is_completed {
            info!("Niveau {} terminé avec succès!", level_manager.get_current_level().unwrap().name);
        }
    }
}