use crate::components::grid::*;
use crate::components::level::*;
use crate::components::robot::*;
use crate::globals::*;
use crate::resources::grid::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// Système principal pour afficher la grille
pub fn display_grid_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid_query: Query<(&Grid, &Robot), With<CurrentLevel>>,
    existing_display: Query<Entity, With<GridDisplay>>,
    mut display_config: ResMut<GridDisplayConfig>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Nettoie l'affichage existant
    for entity in existing_display.iter() {
        commands.entity(entity).despawn();
    }

    // Vérifie qu'on a un niveau actuel avec grille et robot
    let Ok((grid, robot)) = grid_query.single() else {
        return;
    };

    // Obtenir la taille de la fenêtre
    let Ok(window) = window_query.single() else {
        return;
    };

    // Calcule la position de départ pour centrer la grille
    let grid_pixel_width = grid.width as f32 * (TILE_SIZE + TILE_SPACING) - TILE_SPACING;
    let grid_pixel_height = grid.height as f32 * (TILE_SIZE + TILE_SPACING) - TILE_SPACING;

    // Ajuster le centre de la grille en tenant compte du panel de gauche et de l'UI en bas
    let available_width = window.width() - display_config.left_panel_width;
    let ui_height = 250.0; // Hauteur réduite de l'UI
    let available_height = window.height() - ui_height;

    // Centrer dans l'espace disponible
    let center_x = display_config.left_panel_width + available_width / 2.0;
    let center_y = available_height / 2.0;

    // Convertir en coordonnées Bevy (centrées)
    display_config.grid_center.x = center_x - window.width() / 2.0;
    display_config.grid_center.y = (window.height() / 2.0) - center_y;

    let start_x = display_config.grid_center.x - grid_pixel_width / 2.0;
    let start_y = display_config.grid_center.y + grid_pixel_height / 2.0;

    // Crée les meshes réutilisables
    let tile_mesh = meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE));
    let star_mesh = meshes.add(RegularPolygon::new(STAR_SIZE, 5)); // Étoile à 5 branches
    let robot_mesh = meshes.add(Triangle2d::new(
        Vec2::Y * ROBOT_SIZE / 2.0,                      // Pointe vers le haut
        Vec2::new(-ROBOT_SIZE / 3.0, -ROBOT_SIZE / 2.0), // Coin gauche
        Vec2::new(ROBOT_SIZE / 3.0, -ROBOT_SIZE / 2.0),  // Coin droit
    ));

    // Affiche chaque tuile (seulement celles qui existent)
    for (_, tile_opt) in grid.tiles.iter().enumerate() {
        if let Some(tile) = tile_opt {
            let world_x = start_x + tile.x as f32 * (TILE_SIZE + TILE_SPACING) + TILE_SIZE / 2.0;
            let world_y = start_y - tile.y as f32 * (TILE_SIZE + TILE_SPACING) - TILE_SIZE / 2.0;

            // Crée la tuile
            commands.spawn((
                Mesh2d(tile_mesh.clone()),
                MeshMaterial2d(materials.add(tile.color.to_bevy_color())),
                Transform::from_xyz(world_x, world_y, 0.0),
                GridTile {
                    grid_x: tile.x,
                    grid_y: tile.y,
                },
                GridDisplay,
            ));

            // Ajoute une étoile si nécessaire
            if tile.has_star && !tile.star_collected {
                commands.spawn((
                    Mesh2d(star_mesh.clone()),
                    MeshMaterial2d(materials.add(COLOR_STAR)),
                    Transform::from_xyz(world_x, world_y, 1.0), // Z=1 pour être au-dessus de la tuile
                    GridStar {
                        grid_x: tile.x,
                        grid_y: tile.y,
                    },
                    GridDisplay,
                ));
            }
        }
    }

    // Affiche le robot
    let robot_world_x = start_x + robot.x as f32 * (TILE_SIZE + TILE_SPACING) + TILE_SIZE / 2.0;
    let robot_world_y = start_y - robot.y as f32 * (TILE_SIZE + TILE_SPACING) - TILE_SIZE / 2.0;

    commands.spawn((
        Mesh2d(robot_mesh),
        MeshMaterial2d(materials.add(COLOR_ROBOT)),
        Transform::from_xyz(robot_world_x, robot_world_y, 2.0) // Z=2 pour être au-dessus de tout
            .with_rotation(Quat::from_rotation_z(robot.direction.to_rotation())),
        GridRobot,
        GridDisplay,
    ));
}
