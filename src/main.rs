use bevy::diagnostic::FrameCount;
use crate::events::execution::*;
use crate::events::game::*;
use crate::events::level::*;
use crate::events::player::*;
use crate::events::robot::*;
use crate::plugins::grid::*;
use crate::plugins::loading::*;
use crate::plugins::menu::*;
use crate::plugins::player::*;
use crate::plugins::tutorial::*;
use crate::plugins::timer::*;
use crate::resources::execution::*;
use crate::resources::game::*;
use crate::resources::level::*;
use crate::resources::loading::*;
use crate::resources::player::*;
use crate::states::game::*;
use crate::systems::ui::EguiUIPlugin;
use crate::systems::time_up::TimeUpPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};

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
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Test technique".into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                visible: false,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                ..default()
            }),
            ..default()
        }),))
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
            LevelLoadingPlugin, // Plugin de chargement des niveaux
            MenuPlugin,         // Plugin de menu et auto-start
            TutorialPlugin,     // Plugin pour le tutoriel
            PlayerInfoPlugin,   // Plugin pour la saisie des infos joueur
            GridDisplayPlugin,  // Plugin d'affichage de la grille
            EguiUIPlugin,       // Plugin d'édition d'instructions
            TimerPlugin,        // Plugin de gestion des timers
            TimeUpPlugin,       // Plugin pour l'écran de fin
        ))
        .add_systems(Update, make_visible)
        .run();
}

fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.visible = true;
    }
}