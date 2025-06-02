use crate::components::grid::*;
use crate::components::level::*;
use crate::components::robot::*;
use crate::events::execution::*;
use crate::events::game::*;
use crate::events::level::*;
use crate::events::player::*;
use crate::events::robot::*;
use crate::plugins::grid::*;
use crate::resources::execution::*;
use crate::resources::game::*;
use crate::resources::grid::*;
use crate::resources::level::*;
use crate::resources::player::*;
use crate::states::game::*;
use crate::structs::controls::*;
use crate::structs::tile::*;
use bevy::prelude::*;

mod components;
mod events;
mod globals;
mod levels;
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

// Fonction utilitaire pour créer un niveau de test
pub fn create_test_level(mut commands: Commands) {
    let test_tiles = vec![
        Tile {
            x: 0,
            y: 0,
            color: TileColor::Gray,
            has_star: false,
            star_collected: false,
        },
        Tile {
            x: 1,
            y: 0,
            color: TileColor::Red,
            has_star: true,
            star_collected: false,
        },
        Tile {
            x: 2,
            y: 0,
            color: TileColor::Gray,
            has_star: false,
            star_collected: false,
        },
        Tile {
            x: 3,
            y: 0,
            color: TileColor::Blue,
            has_star: false,
            star_collected: false,
        },
        Tile {
            x: 0,
            y: 1,
            color: TileColor::Gray,
            has_star: false,
            star_collected: false,
        },
        Tile {
            x: 1,
            y: 1,
            color: TileColor::Gray,
            has_star: false,
            star_collected: false,
        },
        Tile {
            x: 2,
            y: 1,
            color: TileColor::Green,
            has_star: true,
            star_collected: false,
        },
        Tile {
            x: 3,
            y: 1,
            color: TileColor::Gray,
            has_star: false,
            star_collected: false,
        },
        Tile {
            x: 0,
            y: 2,
            color: TileColor::Green,
            has_star: false,
            star_collected: false,
        },
        Tile {
            x: 1,
            y: 2,
            color: TileColor::Gray,
            has_star: false,
            star_collected: false,
        },
        Tile {
            x: 2,
            y: 2,
            color: TileColor::Gray,
            has_star: false,
            star_collected: false,
        },
        Tile {
            x: 3,
            y: 2,
            color: TileColor::Red,
            has_star: true,
            star_collected: false,
        },
    ];

    let test_grid = Grid {
        width: 4,
        height: 3,
        tiles: test_tiles,
    };

    let test_robot = Robot {
        x: 0,
        y: 0,
        direction: Direction::East,
        start_x: 0,
        start_y: 0,
        start_direction: Direction::East,
    };

    commands.spawn((test_grid, test_robot, CurrentLevel));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(PlayerInfo::default())
        .insert_resource(GameTimer::new(20.0))
        .insert_resource(LevelManager::new())
        .insert_resource(ExecutionEngine::new(1.0))
        .add_event::<SwitchLevelEvent>()
        .add_event::<StartExecutionEvent>()
        .add_event::<PauseExecutionEvent>()
        .add_event::<StopExecutionEvent>()
        .add_event::<ResetRobotEvent>()
        .add_event::<StarCollectedEvent>()
        .add_event::<TimeUpEvent>()
        .add_event::<PlayerInfoCompleteEvent>()
        .add_plugins(GridDisplayPlugin)
        .add_systems(Startup, create_test_level)
        // Systems à ajouter :
        // - update_timer_system
        // - check_time_up_system
        // - load_levels_system
        // - player_info_input_system
        // - execution_system
        // - robot_movement_system
        // - star_collection_system
        // - ui_systems...
        .run();
}
