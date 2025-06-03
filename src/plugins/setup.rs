use crate::events::execution::{PauseExecutionEvent, StartExecutionEvent, StopExecutionEvent};
use crate::events::game::TimeUpEvent;
use crate::events::level::{StarCollectedEvent, SwitchLevelEvent};
use crate::events::player::PlayerInfoCompleteEvent;
use crate::events::robot::ResetRobotEvent;
use crate::globals::TEST_DURATION;
use crate::resources::execution::ExecutionEngine;
use crate::resources::game::GameTimer;
use crate::resources::level::LevelManager;
use crate::resources::loading::LoadingState;
use crate::resources::player::PlayerInfo;
use crate::states::game::GameState;
use bevy::diagnostic::FrameCount;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins.set(WindowPlugin {
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
        }),));
        app.init_state::<GameState>();
        app.insert_resource(PlayerInfo::default());
        app.insert_resource(GameTimer::new(TEST_DURATION));
        app.insert_resource(LevelManager::new());
        app.insert_resource(LoadingState::default());
        app.insert_resource(ExecutionEngine::new());
        app.add_event::<SwitchLevelEvent>();
        app.add_event::<StartExecutionEvent>();
        app.add_event::<StarCollectedEvent>();
        app.add_event::<PauseExecutionEvent>();
        app.add_event::<StopExecutionEvent>();
        app.add_event::<ResetRobotEvent>();
        app.add_event::<TimeUpEvent>();
        app.add_event::<PlayerInfoCompleteEvent>();
        app.add_systems(Update, make_visible);
    }
}

fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.visible = true;
    }
}
