use bevy::prelude::*;
use crate::components::grid::{Grid, GridDisplay, GridStar};
use crate::components::level::CurrentLevel;

pub 
// Système pour mettre à jour l'affichage des étoiles collectées
fn update_stars_display_system(
    mut commands: Commands,
    star_query: Query<(Entity, &GridStar), With<GridDisplay>>,
    grid_query: Query<&Grid, With<CurrentLevel>>,
) {
    let Ok(grid) = grid_query.single() else {
        return;
    };

    // Pour chaque étoile affichée, vérifie si elle a été collectée
    for (star_entity, grid_star) in star_query.iter() {
        if let Some(tile) = grid.tiles.iter()
            .find(|t| t.x == grid_star.grid_x && t.y == grid_star.grid_y) {

            if tile.star_collected {
                // Supprime l'étoile de l'affichage
                commands.entity(star_entity).despawn();
            }
        }
    }
}