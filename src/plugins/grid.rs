use crate::events::level::SwitchLevelEvent;
use crate::resources::grid::GridDisplayConfig;
use crate::setup_camera;
use crate::systems::display::update_stars_display_system;
use crate::systems::grid::display_grid_system;
use crate::systems::level::handle_level_switch_system;
use crate::systems::robot::update_robot_position_system;
use bevy::prelude::*;

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
            );
    }
}
