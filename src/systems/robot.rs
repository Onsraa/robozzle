use crate::components::grid::{Grid, GridRobot};
use crate::components::level::CurrentLevel;
use crate::components::robot::Robot;
use crate::globals::*;
use crate::resources::grid::GridDisplayConfig;
use bevy::prelude::*;

// Système pour mettre à jour la position du robot pendant l'exécution
pub fn update_robot_position_system(
    mut robot_query: Query<&mut Transform, With<GridRobot>>,
    robot_data_query: Query<&Robot, With<CurrentLevel>>,
    grid_query: Query<&Grid, With<CurrentLevel>>,
    display_config: Res<GridDisplayConfig>,
) {
    let Ok(mut robot_transform) = robot_query.single_mut() else {
        return;
    };

    let Ok(robot_data) = robot_data_query.single() else {
        return;
    };

    let Ok(grid) = grid_query.single() else {
        return;
    };

    // Recalcule la position du robot
    let grid_pixel_width = grid.width as f32 * (TILE_SIZE + TILE_SPACING) - TILE_SPACING;
    let grid_pixel_height = grid.height as f32 * (TILE_SIZE + TILE_SPACING) - TILE_SPACING;

    let start_x = display_config.grid_center.x - grid_pixel_width / 2.0;
    let start_y = display_config.grid_center.y + grid_pixel_height / 2.0;

    let robot_world_x =
        start_x + robot_data.x as f32 * (TILE_SIZE + TILE_SPACING) + TILE_SIZE / 2.0;
    let robot_world_y =
        start_y - robot_data.y as f32 * (TILE_SIZE + TILE_SPACING) - TILE_SIZE / 2.0;

    robot_transform.translation.x = robot_world_x;
    robot_transform.translation.y = robot_world_y;
    robot_transform.rotation = Quat::from_rotation_z(robot_data.direction.to_rotation());
}