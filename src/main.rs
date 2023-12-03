mod pause;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        // default plugins
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin::default()))
        .add_plugins(EguiPlugin)
        // states
        .add_state::<GameState>()
        // custom plugins
        .add_plugins(pause::PausePlugin)
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