use bevy::prelude::*;
use crate::structs::tile::Tile;

#[derive(Component)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
}

impl Grid {
    fn get_tile_at(&self, x: i32, y: i32) -> Option<&Tile> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize;
        self.tiles.get(index)
    }

    fn get_tile_at_mut(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize;
        self.tiles.get_mut(index)
    }

    fn is_valid_position(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }
}

// Composants pour identifier les éléments visuels de la grille
#[derive(Component)]
pub struct GridTile {
    pub grid_x: i32,
    pub grid_y: i32,
}

#[derive(Component)]
pub struct GridStar {
    pub grid_x: i32,
    pub grid_y: i32,
}

#[derive(Component)]
pub struct GridRobot;

#[derive(Component)]
pub struct GridDisplay; // Marker pour toute la grille affichée
