use crate::components::grid::*;
use crate::components::level::*;
use crate::components::robot::*;
use crate::events::level::StarCollectedEvent;
use crate::resources::execution::*;
use crate::resources::level::*;
use crate::states::game::GameState;
use crate::structs::controls::*;
use crate::structs::tile::TileColor;
use bevy::prelude::*;

// Système principal d'exécution des instructions
pub fn execution_system(
    time: Res<Time>,
    mut execution_engine: ResMut<ExecutionEngine>,
    mut robot_query: Query<&mut Robot, With<CurrentLevel>>,
    mut grid_query: Query<&mut Grid, With<CurrentLevel>>,
    mut level_manager: ResMut<LevelManager>,
    mut star_events: EventWriter<StarCollectedEvent>,
) {
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

    let level_id = current_level.id;

    // Obtient l'état du problème pour accéder aux fonctions
    let Some(problem_state) = level_manager.get_problem_state(level_id) else {
        return;
    };

    // Boucle pour skip les instructions conditionnelles non satisfaites
    loop {
        let current_function = execution_engine.get_current_function();
        let current_instruction_index = execution_engine.get_current_instruction();

        if current_function >= problem_state.functions.len() {
            execution_engine.stop();
            return;
        }

        let function = &problem_state.functions[current_function];

        if current_instruction_index >= function.len() {
            if !execution_engine.return_from_function() {
                return;
            }
            continue;
        }

        let instruction = function[current_instruction_index].clone();

        let should_skip = match &instruction {
            Instruction::ConditionalRed(_) => {
                if let Some(tile) = grid.get_tile_at(robot.x, robot.y) {
                    tile.color != TileColor::Red
                } else {
                    true
                }
            }
            Instruction::ConditionalGreen(_) => {
                if let Some(tile) = grid.get_tile_at(robot.x, robot.y) {
                    tile.color != TileColor::Green
                } else {
                    true
                }
            }
            Instruction::ConditionalBlue(_) => {
                if let Some(tile) = grid.get_tile_at(robot.x, robot.y) {
                    tile.color != TileColor::Blue
                } else {
                    true
                }
            }
            _ => false,
        };

        if should_skip {
            execution_engine.advance_instruction();
            continue;
        }

        match execute_instruction(
            instruction,
            &mut robot,
            &mut grid,
            &mut execution_engine,
            &level_manager,
            &mut star_events,
            level_id,
        ) {
            Ok(()) => {
                break; 
            }
            Err(error_msg) => {
                execution_engine.set_error(error_msg);
                robot.reset_to_start();
                break;
            }
        }
    }
}

fn execute_instruction(
    instruction: Instruction,
    robot: &mut Robot,
    grid: &mut Grid,
    execution_engine: &mut ExecutionEngine,
    level_manager: &LevelManager,
    star_events: &mut EventWriter<StarCollectedEvent>,
    level_id: usize,
) -> Result<(), String> {
    match instruction {
        Instruction::Forward => {
            let (dx, dy) = robot.direction.get_offset();
            let new_x = robot.x + dx;
            let new_y = robot.y + dy;

            if !grid.is_valid_position(new_x, new_y) {
                return Err("Vous êtes en dehors du puzzle".to_string());
            }

            robot.x = new_x;
            robot.y = new_y;

            if let Some(tile) = grid.get_tile_at_mut(new_x, new_y) {
                if tile.has_star && !tile.star_collected {
                    tile.star_collected = true;
                    star_events.write(StarCollectedEvent { x: new_x, y: new_y });
                    info!("Étoile collectée à ({}, {})", new_x, new_y);
                }
            }

            execution_engine.advance_instruction();
            Ok(())
        }

        Instruction::TurnLeft => {
            robot.turn_left();
            execution_engine.advance_instruction();
            Ok(())
        }

        Instruction::TurnRight => {
            robot.turn_right();
            execution_engine.advance_instruction();
            Ok(())
        }

        Instruction::CallFunction(function_id) => {
            // Vérifier que la fonction existe
            if let Some(problem_state) = level_manager.get_problem_state(level_id) {
                if function_id < problem_state.functions.len() {
                    info!(
                        "Appel de fonction {} depuis fonction {} instruction {}",
                        function_id,
                        execution_engine.get_current_function(),
                        execution_engine.get_current_instruction()
                    );
                    execution_engine.call_function(function_id);
                    // Ne pas avancer l'instruction ici, l'exécution continue dans la nouvelle fonction
                    Ok(())
                } else {
                    Err(format!("Fonction {} n'existe pas", function_id))
                }
            } else {
                Err("Aucun état de problème trouvé".to_string())
            }
        }

        Instruction::ConditionalRed(inner_instruction) => {
            if let Some(tile) = grid.get_tile_at(robot.x, robot.y) {
                if tile.color == TileColor::Red {
                    execute_instruction(
                        *inner_instruction,
                        robot,
                        grid,
                        execution_engine,
                        level_manager,
                        star_events,
                        level_id,
                    )?;
                } else {
                    execution_engine.advance_instruction();
                }
            } else {
                execution_engine.advance_instruction();
            }
            Ok(())
        }

        Instruction::ConditionalGreen(inner_instruction) => {
            if let Some(tile) = grid.get_tile_at(robot.x, robot.y) {
                if tile.color == TileColor::Green {
                    execute_instruction(
                        *inner_instruction,
                        robot,
                        grid,
                        execution_engine,
                        level_manager,
                        star_events,
                        level_id,
                    )?;
                } else {
                    execution_engine.advance_instruction();
                }
            } else {
                execution_engine.advance_instruction();
            }
            Ok(())
        }

        Instruction::ConditionalBlue(inner_instruction) => {
            if let Some(tile) = grid.get_tile_at(robot.x, robot.y) {
                if tile.color == TileColor::Blue {
                    execute_instruction(
                        *inner_instruction,
                        robot,
                        grid,
                        execution_engine,
                        level_manager,
                        star_events,
                        level_id,
                    )?;
                } else {
                    execution_engine.advance_instruction();
                }
            } else {
                execution_engine.advance_instruction();
            }
            Ok(())
        }

        Instruction::Noop => {
            execution_engine.advance_instruction();
            Ok(())
        }
    }
}

// Système pour mettre à jour le compteur d'étoiles et arrêter si toutes collectées
pub fn update_star_counter_system(
    mut star_events: EventReader<StarCollectedEvent>,
    mut level_manager: ResMut<LevelManager>,
    mut execution_engine: ResMut<ExecutionEngine>,
    grid_query: Query<&Grid, With<CurrentLevel>>,
) {
    for event in star_events.read() {
        if let Some(current_level) = level_manager.get_current_level() {
            let level_id = current_level.id;
            let total_stars = current_level.total_stars;

            // Compter les étoiles actuellement collectées dans la grille
            if let Ok(grid) = grid_query.single() {
                let stars_collected = grid
                    .tiles
                    .iter()
                    .filter_map(|tile_opt| tile_opt.as_ref())
                    .filter(|tile| tile.has_star && tile.star_collected)
                    .count();

                if let Some(problem_state) = level_manager.get_problem_state_mut(level_id) {
                    problem_state.stars_collected = stars_collected;
                    info!("Étoiles collectées: {}/{}", stars_collected, total_stars);

                    if stars_collected >= total_stars {
                        problem_state.is_completed = true;
                        execution_engine.stop();
                        info!("Toutes les étoiles collectées! Arrêt de l'exécution.");
                    }
                }
            }
        }
    }
}

// Système pour vérifier si le puzzle est résolu
pub fn check_completion_system(
    grid_query: Query<&Grid, With<CurrentLevel>>,
    mut level_manager: ResMut<LevelManager>,
    execution_engine: Res<ExecutionEngine>,
    mut next_state: ResMut<NextState<GameState>>,
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

    // Compte les étoiles collectées directement depuis la grille
    let stars_collected = grid
        .tiles
        .iter()
        .filter_map(|tile_opt| tile_opt.as_ref())
        .filter(|tile| tile.has_star && tile.star_collected)
        .count();

    // Met à jour l'état du problème
    if let Some(problem_state) = level_manager.get_problem_state_mut(level_id) {
        problem_state.stars_collected = stars_collected;
        problem_state.check_completion(total_stars);

        if problem_state.is_completed && !problem_state.completion_time_recorded {
            // Enregistrer le temps de complétion
            problem_state.completion_time_recorded = true;
            problem_state.record_completion_time();
        }
    }

    // Vérifier si tous les niveaux sont complétés (seulement en mode normal)
    if level_manager.get_current_level_type() == crate::resources::level::LevelType::Normal {
        if level_manager.are_all_levels_completed() {
            info!("Tous les puzzles sont complétés! Fin du jeu.");
            next_state.set(GameState::TimeUp);
        }
    }
}
