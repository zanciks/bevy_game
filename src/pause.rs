use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::GameState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, capture_input)
            .add_systems(Update, (pause_menu).
                distributive_run_if(in_state(GameState::Paused)));
    }
}

pub fn capture_input(
    keys: Res<Input<KeyCode>>,
    ui_state: Res<State<GameState>>, // read current state
    mut next_state: ResMut<NextState<GameState>> // set the state for the next update
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(match ui_state.get() {
            GameState::Paused => GameState::Playing,
            _ => GameState::Paused
        })
    }
}

pub fn pause_menu(mut contexts: EguiContexts, bevy_window: Query<&Window>, time: Res<Time>) {
    let width = bevy_window.single().width() * 0.9;
    let height = bevy_window.single().height() * 0.9;
    let window_size = egui::Vec2::new(width, height);

    let window = egui::Window::new("title")
        .title_bar(false)
        .fixed_size(window_size)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO);

    window.show(contexts.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
            ui.label("");
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            ui.label(format!("Game Runtime: {}", format_duration(time.elapsed_seconds())));
        });
    });
}

fn format_duration(seconds: f32) -> String {
    let new_seconds = seconds as u32;

    let hours = new_seconds / 3600;
    let minutes = (new_seconds % 3600) / 60;
    let seconds = new_seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}