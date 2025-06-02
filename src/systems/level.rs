use crate::components::grid::*;
use crate::components::level::*;
use crate::components::robot::*;
use crate::events::level::*;
use crate::resources::grid::*;
use crate::resources::level::*;
use crate::states::game::GameState;
use bevy::prelude::*;

// Système pour gérer le changement de niveau
pub fn handle_level_switch_system(
    mut level_switch_events: EventReader<SwitchLevelEvent>,
    mut display_config: ResMut<GridDisplayConfig>,
    mut level_manager: ResMut<LevelManager>,
    mut commands: Commands,
    existing_display: Query<Entity, With<GridDisplay>>,
    mut grid_query: Query<(&mut Grid, &mut Robot), With<CurrentLevel>>,
) {
    for event in level_switch_events.read() {
        // Nettoie l'affichage actuel
        for entity in existing_display.iter() {
            commands.entity(entity).despawn();
        }

        // Change le niveau dans le manager
        level_manager.switch_to_level(event.0);

        // Met à jour la grille et le robot avec les nouvelles données
        if let (Some(level_data), Ok((mut grid, mut robot))) =
            (level_manager.get_current_level(), grid_query.single_mut())
        {
            // Met à jour la grille
            grid.width = level_data.width;
            grid.height = level_data.height;
            grid.tiles = level_data.tiles.clone();

            // Remet les étoiles dans leur état initial
            for tile_opt in &mut grid.tiles {
                if let Some(tile) = tile_opt {
                    tile.star_collected = false;
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

// Système pour lancer automatiquement le premier exercice quand on arrive au menu
pub fn auto_start_first_level_system(
    mut commands: Commands,
    level_manager: Res<LevelManager>,
    mut next_state: ResMut<NextState<GameState>>,
    existing_level: Query<Entity, With<CurrentLevel>>,
) {
    // Nettoie le niveau actuel s'il existe
    for entity in existing_level.iter() {
        commands.entity(entity).despawn_recursive();
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

        // Passe à l'état d'édition
        next_state.set(GameState::Editing);

        info!("Niveau {} prêt - transition vers Editing", first_level.name);
    } else {
        warn!("Aucun niveau disponible pour le lancement automatique");
        // Retour au chargement en cas de problème
        next_state.set(GameState::Loading);
    }
}

// Système pour afficher les informations du niveau actuel
pub fn display_level_info_system(
    level_manager: Res<LevelManager>,
    mut commands: Commands,
    level_info_query: Query<Entity, With<LevelInfoDisplay>>,
) {
    // Nettoie l'affichage précédent
    for entity in level_info_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if let Some(current_level) = level_manager.get_current_level() {
        // Affiche le nom du niveau en haut à gauche
        commands.spawn((
            Text::new(format!("Niveau: {}", current_level.name)),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(20.0),
                ..default()
            },
            LevelInfoDisplay,
        ));

        // Affiche le nombre d'étoiles en haut à droite
        if let Some(problem_state) = level_manager.get_problem_state(current_level.id) {
            commands.spawn((
                Text::new(format!("Étoiles: {}/{}",
                                  problem_state.stars_collected,
                                  current_level.total_stars)),
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(20.0),
                    top: Val::Px(20.0),
                    ..default()
                },
                LevelInfoDisplay,
            ));
        }

        // Affiche les fonctions disponibles
        let functions_text = format!("Fonctions: {}",
                                     current_level.function_limits.iter()
                                         .enumerate()
                                         .map(|(i, &limit)| format!("F{}: {}", i+1, limit))
                                         .collect::<Vec<_>>()
                                         .join(" | "));

        commands.spawn((
            Text::new(functions_text),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(60.0),
                ..default()
            },
            LevelInfoDisplay,
        ));
    }
}

// Système pour afficher les contrôles disponibles
pub fn display_controls_system(
    mut commands: Commands,
    controls_query: Query<Entity, With<ControlsDisplay>>,
    game_state: Res<State<GameState>>,
) {
    // Ne s'affiche que dans l'état Editing
    if *game_state != GameState::Editing {
        return;
    }

    // Nettoie l'affichage précédent
    for entity in controls_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let controls_text = r#"Contrôles:
[SPACE] - Démarrer/Arrêter l'exécution
[P] - Pause/Reprendre
[R] - Reset robot
[1-9] - Changer de niveau
[ESC] - Quitter"#;

    commands.spawn((
        Text::new(controls_text),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            bottom: Val::Px(20.0),
            ..default()
        },
        ControlsDisplay,
    ));
}

// Composants pour l'UI
#[derive(Component)]
pub struct LevelInfoDisplay;

#[derive(Component)]
pub struct ControlsDisplay;