use bevy::prelude::*;

#[derive(Resource)]
pub struct GridDisplayConfig {
    pub camera_entity: Option<Entity>,
    pub grid_center: Vec2,
    pub left_panel_width: f32,
}

impl Default for GridDisplayConfig {
    fn default() -> Self {
        Self {
            camera_entity: None,
            grid_center: Vec2::ZERO,
            left_panel_width: 0.0,
        }
    }
}