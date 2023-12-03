mod pause;
mod player;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_xpbd_3d::prelude::*;

fn main() {
    App::new()
        // default plugins
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin::default()))
        .add_plugins(EguiPlugin)
        .add_plugins(PhysicsPlugins::default())
        // states
        .add_state::<GameState>()
        // custom plugins and systems
        .add_plugins(pause::PausePlugin)
        .add_plugins(player::PlayerPlugin)
        .add_systems(Update, (update_physics_time, grab_mouse))
        //run
        .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
    QuitMenu
}

fn update_physics_time(
    game_state: Res<State<GameState>>,
    mut physics_time: ResMut<Time<Physics>>,
) {
    match game_state.get() {
        GameState::Playing => physics_time.unpause(),
        _ => physics_time.pause(),
    }
}

fn grab_mouse(
    mut windows: Query<&mut Window>,
    game_state: Res<State<GameState>>,
) {
    let mut window = windows.single_mut();
    match game_state.get() {
        GameState::Playing => {
            window.cursor.visible = false;
            window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
        },
        _ => {
            window.cursor.visible = true;
            window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
        }
    }
}