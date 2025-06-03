use bevy::color::Color;

pub const TEST_DURATION: f32 = 1.0;
pub const TILE_SIZE: f32 = 45.0;               // Taille d'une tuile en pixels (réduit de 60 à 45)
pub const TILE_SPACING: f32 = 2.0;             // Espacement entre les tuiles
pub const STAR_SIZE: f32 = 15.0;               // Taille des étoiles (réduit de 20 à 15)
pub const ROBOT_SIZE: f32 = 22.0;              // Taille du robot (réduit de 30 à 22)

// Couleurs des tuiles
pub const COLOR_GRAY: Color = Color::srgb(0.7, 0.7, 0.7);
pub const COLOR_RED: Color = Color::srgb(0.9, 0.2, 0.2);
pub const COLOR_GREEN: Color = Color::srgb(0.2, 0.9, 0.2);
pub const COLOR_BLUE: Color = Color::srgb(0.2, 0.2, 0.9);
pub const COLOR_STAR: Color = Color::srgb(1.0, 1.0, 0.0);      // Jaune pour les étoiles
pub const COLOR_ROBOT: Color = Color::srgb(0.1, 0.1, 0.1);     // Noir pour le robot