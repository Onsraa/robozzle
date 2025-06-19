use bevy::prelude::*;
use crate::structs::tile::Tile;

#[derive(Component)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Option<Tile>>,  // Option pour supporter les cases vides
}

impl Grid {
    pub fn get_tile_at(&self, x: i32, y: i32) -> Option<&Tile> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize;
        self.tiles.get(index)?.as_ref()  // Double Option : index valide + case non vide
    }

    pub fn get_tile_at_mut(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize;
        self.tiles.get_mut(index)?.as_mut()
    }

    pub fn is_valid_position(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return false;
        }
        // Une position est valide si elle contient une tuile (pas vide)
        self.get_tile_at(x, y).is_some()
    }

    // Vérifie si une position existe dans la grille (sans vérifier si elle a une tuile)
    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    // Obtient l'index d'une position dans le Vec
    fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        if self.is_in_bounds(x, y) {
            Some((y * self.width + x) as usize)
        } else {
            None
        }
    }

    // Définit une tuile à une position (remplace ou crée)
    pub fn set_tile_at(&mut self, x: i32, y: i32, tile: Option<Tile>) {
        if let Some(index) = self.get_index(x, y) {
            if index < self.tiles.len() {
                self.tiles[index] = tile;
            }
        }
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