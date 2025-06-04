use crate::structs::controls::{Direction, Instruction};
use crate::structs::tile::{Tile, TileColor};
use bevy::prelude::*;
use std::fs;

#[derive(Clone)]
pub struct LevelData {
    pub id: usize,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Option<Tile>>,  // Support des cases vides
    pub robot_start_pos: (i32, i32),
    pub robot_start_dir: Direction,
    pub total_stars: usize,
    pub function_limits: Vec<usize>,
}

impl LevelData {
    pub fn from_file(filename: &str, id: usize) -> Result<Self, String> {
        let content = fs::read_to_string(filename)
            .map_err(|e| format!("Erreur lecture fichier {}: {}", filename, e))?;

        let mut name = format!("Problème {}", id + 1);
        let mut width = 0;
        let mut height = 0;
        let mut robot_pos = (0, 0);
        let mut robot_dir = Direction::North;
        let mut function_limits = vec![5];
        let mut tiles: Vec<Option<Tile>> = Vec::new();

        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            if line.starts_with("LEVEL ") {
                name = line[6..].to_string();
            } else if line.starts_with("SIZE ") {
                let parts: Vec<&str> = line[5..].split_whitespace().collect();
                if parts.len() == 2 {
                    width = parts[0].parse().map_err(|_| "Erreur parse width")?;
                    height = parts[1].parse().map_err(|_| "Erreur parse height")?;
                }
            } else if line.starts_with("ROBOT ") {
                let parts: Vec<&str> = line[6..].split_whitespace().collect();
                if parts.len() == 3 {
                    robot_pos.0 = parts[0].parse().map_err(|_| "Erreur parse robot x")?;
                    robot_pos.1 = parts[1].parse().map_err(|_| "Erreur parse robot y")?;
                    robot_dir = Direction::from_str(parts[2]).ok_or("Direction invalide")?;
                }
            } else if line.starts_with("FUNCTIONS ") {
                let parts: Vec<&str> = line[10..].split_whitespace().collect();
                function_limits = parts
                    .iter()
                    .map(|s| s.parse().map_err(|_| "Erreur parse functions"))
                    .collect::<Result<Vec<_>, _>>()?;
            } else if line == "GRID:" {
                i += 1;
                break;
            }
            i += 1;
        }

        // Parse la grille avec support des cases vides
        for y in 0..height {
            if i + y as usize >= lines.len() {
                break;
            }

            let line = lines[i + y as usize];
            let cells: Vec<&str> = line.split_whitespace().collect();

            for x in 0..width {
                let cell = if x < cells.len() as i32 {
                    cells[x as usize]
                } else {
                    "X" // Case vide par défaut si pas assez de données
                };

                if cell == "X" {
                    // Case vide - on ajoute None
                    tiles.push(None);
                } else {
                    // Case normale
                    let has_star = cell.contains('*');
                    let color_char = cell.chars().next().unwrap_or('.');
                    let color = TileColor::from_char(color_char);
                    tiles.push(Some(Tile::new(x, y, color, has_star)));
                }
            }
        }

        let total_stars = tiles.iter()
            .filter_map(|tile_opt| tile_opt.as_ref())  // Filtre les cases vides
            .filter(|tile| tile.has_star)             // Filtre les tuiles avec étoiles
            .count();

        Ok(LevelData {
            id,
            name,
            width,
            height,
            tiles,
            robot_start_pos: robot_pos,
            robot_start_dir: robot_dir,
            total_stars,
            function_limits,
        })
    }
}

#[derive(Clone)]
pub struct ProblemState {
    pub functions: Vec<Vec<Instruction>>,
    pub stars_collected: usize,
    pub is_completed: bool,
    pub completion_time: Option<f32>,  // Temps en secondes pour compléter
    pub start_time: f32,              // Temps de début en secondes
    pub completion_time_recorded: bool, // Pour éviter d'enregistrer plusieurs fois
}

impl ProblemState {
    pub fn new(num_functions: usize) -> Self {
        Self {
            functions: vec![Vec::new(); num_functions],
            stars_collected: 0,
            is_completed: false,
            completion_time: None,
            start_time: 0.0,
            completion_time_recorded: false,
        }
    }

    pub fn reset_stars(&mut self) {
        self.stars_collected = 0;
        // Ne pas réinitialiser is_completed ni completion_time pour garder l'historique
        // Mais reset completion_time_recorded pour permettre de réenregistrer le temps si on refait le niveau
        if !self.is_completed {
            self.completion_time_recorded = false;
        }
    }

    pub fn check_completion(&mut self, total_stars: usize) {
        if self.stars_collected == total_stars && !self.is_completed {
            self.is_completed = true;
        }
    }

    pub fn start_timer(&mut self, current_time: f32) {
        self.start_time = current_time;
        self.completion_time_recorded = false;
    }

    pub fn record_completion_time(&mut self) {
        if !self.completion_time_recorded && self.is_completed {
            // Le temps est calculé dans le système qui appelle cette méthode
            self.completion_time_recorded = true;
        }
    }

    pub fn set_completion_time(&mut self, elapsed: f32) {
        if self.completion_time.is_none() || elapsed < self.completion_time.unwrap() {
            self.completion_time = Some(elapsed);
        }
    }
}