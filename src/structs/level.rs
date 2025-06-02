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
    pub tiles: Vec<Tile>,
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
        let mut tiles = Vec::new();

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

        for y in 0..height {
            if i + y as usize >= lines.len() {
                break;
            }

            let line = lines[i + y as usize];
            let cells: Vec<&str> = line.split_whitespace().collect();

            for (x, cell) in cells.iter().enumerate() {
                if x >= width as usize {
                    break;
                }

                let has_star = cell.contains('*');
                let color_char = cell.chars().next().unwrap_or('.');
                let color = TileColor::from_char(color_char);

                tiles.push(Tile::new(x as i32, y, color, has_star));
            }
        }

        let total_stars = tiles.iter().filter(|t| t.has_star).count();

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
}

impl ProblemState {
    pub fn new(num_functions: usize) -> Self {
        Self {
            functions: vec![Vec::new(); num_functions],
            stars_collected: 0,
            is_completed: false,
        }
    }

    pub fn reset_stars(&mut self) {
        self.stars_collected = 0;
        self.is_completed = false;
    }

    pub fn check_completion(&mut self, total_stars: usize) {
        self.is_completed = self.stars_collected == total_stars;
    }
}
