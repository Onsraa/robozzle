use bevy::prelude::*;

// Resource pour gérer l'état du chargement
#[derive(Resource)]
pub struct LoadingState {
    pub error_message: Option<String>,
    pub levels_path: String,
}

impl Default for LoadingState {
    fn default() -> Self {
        Self {
            error_message: None,
            levels_path: "src/levels".to_string(), 
        }
    }
}