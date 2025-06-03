use crate::plugins::robozzle::RobozzlePlugin;
use bevy::diagnostic::FrameCount;
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

fn main() {
    App::new().add_plugins(RobozzlePlugin).run();
}
