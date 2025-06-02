use crate::components::grid::*;
use crate::components::level::*;
use crate::components::robot::*;
use crate::events::level::*;
use crate::resources::grid::*;
use crate::resources::level::*;
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
            for tile in &mut grid.tiles {
                tile.star_collected = false;
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
