use crate::plugins::robozzle::RobozzlePlugin;
use bevy::prelude::*;

mod components;
mod events;
mod globals;
mod plugins;
mod resources;
mod states;
mod structs;
mod systems;

fn main() {
    App::new().add_plugins(RobozzlePlugin).run();
}
