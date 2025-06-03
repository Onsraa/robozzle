use crate::events::level::SwitchLevelEvent;
use crate::resources::grid::GridDisplayConfig;
use crate::systems::display::update_stars_display_system;
use crate::systems::grid::display_grid_system;
use crate::systems::level::handle_level_switch_system;
use crate::systems::robot::update_robot_position_system;
use crate::systems::execution::{execution_system, update_star_counter_system, check_completion_system};
use crate::states::game::GameState;
use bevy::prelude::*;

// Système pour setup la caméra
fn setup_camera(mut commands: Commands, mut display_config: ResMut<GridDisplayConfig>) {
    let camera_entity = commands.spawn(Camera2d).id();
    display_config.camera_entity = Some(camera_entity);
}

pub struct GridDisplayPlugin;

impl Plugin for GridDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridDisplayConfig>()
            .add_event::<SwitchLevelEvent>()
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (
                    display_grid_system,
                    update_robot_position_system,
                    update_stars_display_system,
                    handle_level_switch_system,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    execution_system,
                    update_star_counter_system,
                    check_completion_system,
                )
                    .chain()
                    .run_if(in_state(GameState::Editing).or(in_state(GameState::Tutorial))),
            );
    }
}