use crate::events::execution::*;
use crate::events::game::*;
use crate::events::level::*;
use crate::events::player::*;
use crate::events::robot::*;
use crate::plugins::grid::*;
use crate::plugins::loading::*;
use crate::plugins::menu::*;
use crate::resources::execution::*;
use crate::resources::game::*;
use crate::resources::grid::*;
use crate::resources::level::*;
use crate::resources::loading::*;
use crate::resources::player::*;
use crate::states::game::*;
use bevy::prelude::*;

mod components;
mod events;
mod globals;
mod plugins;
mod resources;
mod results;
mod states;
mod structs;
mod systems;

// Système pour setup la caméra
fn setup_camera(mut commands: Commands, mut display_config: ResMut<GridDisplayConfig>) {
    let camera_entity = commands.spawn(Camera2d).id();
    display_config.camera_entity = Some(camera_entity);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(PlayerInfo::default())
        .insert_resource(GameTimer::new(20.0))
        .insert_resource(LevelManager::new())
        .insert_resource(LoadingState::default())
        .insert_resource(ExecutionEngine::new(1.0))
        // Events
        .add_event::<SwitchLevelEvent>()
        .add_event::<StartExecutionEvent>()
        .add_event::<PauseExecutionEvent>()
        .add_event::<StopExecutionEvent>()
        .add_event::<ResetRobotEvent>()
        .add_event::<StarCollectedEvent>()
        .add_event::<TimeUpEvent>()
        .add_event::<PlayerInfoCompleteEvent>()
        // Plugins personnalisés
        .add_plugins((
            LevelLoadingPlugin,  // Plugin de chargement des niveaux
            MenuPlugin,          // Plugin de menu et auto-start
            GridDisplayPlugin,   // Plugin d'affichage de la grille
        ))
        .run();
}