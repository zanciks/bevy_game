use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::GameState;

#[derive(Resource)]
pub struct Settings {
    pub mouse_sensitivity: f32 // value from 0.1 to 10.0
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 5.0,
        }
    }
}

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, settings_menu.run_if(in_state(GameState::SettingsMenu)));
    }
}

fn settings_menu(
    mut contexts: EguiContexts,
    bevy_window: Query<&Window>,
    mut next_state: ResMut<NextState<GameState>>,
    mut settings: ResMut<Settings>,
) {
    let width = bevy_window.single().width() * 0.9;
    let height = bevy_window.single().height() * 0.9;
    let window_size = egui::Vec2::new(width, height);

    let settings_window = egui::Window::new("Settings")
        .collapsible(false)
        .movable(false)
        .resizable(false)
        .fixed_size(window_size)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO);

    settings_window.show(contexts.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
            ui.add(egui::Slider::new(&mut settings.mouse_sensitivity, 0.1..=10.0).text("Mouse Sensitivity"));
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            if ui.button("Return").clicked() {
                next_state.set(GameState::Paused);
            }
        });
    });
}