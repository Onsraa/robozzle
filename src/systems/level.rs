use crate::components::grid::*;
use crate::components::level::*;
use crate::components::robot::*;
use crate::events::level::*;
use crate::resources::grid::*;
use crate::resources::level::*;
use crate::resources::timer::LevelTimer;
use crate::states::game::GameState;
use bevy::prelude::*;

// Système pour gérer le changement de niveau
pub fn handle_level_switch_system(
    mut level_switch_events: EventReader<SwitchLevelEvent>,
    mut level_manager: ResMut<LevelManager>,
    mut commands: Commands,
    existing_display: Query<Entity, With<GridDisplay>>,
    mut grid_query: Query<(&mut Grid, &mut Robot), With<CurrentLevel>>,
    mut level_timer: ResMut<LevelTimer>,
) {
    for event in level_switch_events.read() {
        for entity in existing_display.iter() {
            commands.entity(entity).despawn();
        }

        // Change le niveau dans le manager
        level_manager.switch_to_level(event.0);

        // Reset le timer de niveau
        level_timer.reset();

        // Récupérer les données du niveau en deux étapes pour éviter les conflits d'emprunt
        let level_data_opt = level_manager.get_current_level().cloned();

        if let Some(level_data) = level_data_opt {
            let level_id = level_data.id;

            // Enregistrer le temps de début pour ce niveau
            if let Some(problem_state) = level_manager.get_problem_state_mut(level_id) {
                problem_state.start_timer(0.0);
            }

            // Met à jour la grille et le robot avec les nouvelles données
            if let Ok((mut grid, mut robot)) = grid_query.single_mut() {
                // Vérifier si le niveau est déjà complété
                let is_completed = level_manager.get_problem_state(level_id)
                    .map(|state| state.is_completed)
                    .unwrap_or(false);

                // Met à jour la grille
                grid.width = level_data.width;
                grid.height = level_data.height;
                grid.tiles = level_data.tiles.clone();

                // Si le niveau est complété, marquer toutes les étoiles comme collectées
                // Sinon, les remettre dans leur état initial
                for tile_opt in &mut grid.tiles {
                    if let Some(tile) = tile_opt {
                        tile.star_collected = is_completed && tile.has_star;
                    }
                }

                // Reset le robot à sa position de départ
                robot.x = level_data.robot_start_pos.0;
                robot.y = level_data.robot_start_pos.1;
                robot.direction = level_data.robot_start_dir;
                robot.start_x = level_data.robot_start_pos.0;
                robot.start_y = level_data.robot_start_pos.1;
                robot.start_direction = level_data.robot_start_dir;
            }
        }
    }
}

// Système pour lancer automatiquement le premier exercice quand on arrive au menu
pub fn auto_start_first_level_system(
    mut commands: Commands,
    level_manager: Res<LevelManager>,
    mut next_state: ResMut<NextState<GameState>>,
    existing_level: Query<Entity, With<CurrentLevel>>,
) {
    // Nettoie le niveau actuel s'il existe
    for entity in existing_level.iter() {
        commands.entity(entity).despawn();
    }

    // Vérifie qu'on a au moins un niveau
    if let Some(first_level) = level_manager.get_current_level() {
        info!("Lancement automatique du niveau: {}", first_level.name);

        // Crée la grille à partir des données du niveau
        let grid = Grid {
            width: first_level.width,
            height: first_level.height,
            tiles: first_level.tiles.clone(),
        };

        // Crée le robot à sa position de départ
        let robot = Robot::new(
            first_level.robot_start_pos.0,
            first_level.robot_start_pos.1,
            first_level.robot_start_dir,
        );

        // Spawn l'entité avec le niveau actuel
        commands.spawn((
            grid,
            robot,
            CurrentLevel,
        ));

        // Passe à l'état d'édition ou tutoriel selon le type de niveau
        if level_manager.get_current_level_type() == LevelType::Tutorial {
            next_state.set(GameState::Tutorial);
        } else {
            next_state.set(GameState::Editing);
        }

        info!("Niveau {} prêt - transition vers {:?}", first_level.name, 
              if level_manager.get_current_level_type() == LevelType::Tutorial { "Tutorial" } else { "Editing" });
    } else {
        warn!("Aucun niveau disponible pour le lancement automatique");
        // Retour au chargement en cas de problème
        next_state.set(GameState::Loading);
    }
}

// Ajoutez ce système de nettoyage
pub fn cleanup_current_level(
    mut commands: Commands,
    level_query: Query<Entity, With<CurrentLevel>>,
    grid_display_query: Query<Entity, With<GridDisplay>>,
) {
    // Nettoyer le niveau actuel
    for entity in level_query.iter() {
        commands.entity(entity).despawn();
    }

    // Nettoyer l'affichage de la grille
    for entity in grid_display_query.iter() {
        commands.entity(entity).despawn();
    }
}