use bevy::prelude::*;

#[derive(Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub color: TileColor,
    pub has_star: bool,
    pub star_collected: bool,
}

impl Tile {
    pub fn new(x: i32, y: i32, color: TileColor, has_star: bool) -> Self {
        Self {
            x,
            y,
            color,
            has_star,
            star_collected: false,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
#[derive(Debug)]
pub enum TileColor {
    #[default]
    Gray,
    Red,
    Green,
    Blue,
}

impl TileColor {
    // Parse depuis caractère (pour format GRID)
    pub fn from_char(c: char) -> Self {
        match c {
            'R' => TileColor::Red,
            'G' => TileColor::Green,
            'B' => TileColor::Blue,
            _ => TileColor::Gray, // '.' ou autre = gris
        }
    }

    // Convertit vers une couleur Bevy pour l'affichage
    pub fn to_bevy_color(self) -> Color {
        match self {
            TileColor::Gray => Color::srgb(0.7, 0.7, 0.7),
            TileColor::Red => Color::srgb(0.9, 0.2, 0.2),
            TileColor::Green => Color::srgb(0.2, 0.9, 0.2),
            TileColor::Blue => Color::srgb(0.2, 0.2, 0.9),
        }
    }
}
