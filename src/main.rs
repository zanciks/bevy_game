mod ui;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        // default plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // egui widgets
        .add_systems(Update, ui_example_system)
        //run
        .run();
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}