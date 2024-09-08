use crate::app::state::{MainWindow, StateApp, Transition::*};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub fn UI(mut contexts: EguiContexts, keys: Res<ButtonInput<KeyCode>>) {
    //egui::SidePanel::left("my_left_panel").show(contexts.ctx_mut(), |ui| {
    //    ui.label("Files/tree here");
    //});

    //implement  state machine like in video
    let mut window = MainWindow::new();
    let mut ModLabel = String::from("");

    //status bar with states
    if keys.just_pressed(KeyCode::KeyI) {
        window.change(InsertT);
    }

    match window.state {
        StateApp::Normal => ModLabel = String::from("Normal"),
        StateApp::Visual => ModLabel = String::from("Visual"),
        StateApp::Insert => ModLabel = String::from("Insert"),
        StateApp::Replace => ModLabel = String::from("Replace"),
    }

    egui::TopBottomPanel::bottom("RustLine").show(contexts.ctx_mut(), |ui| {
        ui.label("[".to_owned() + &ModLabel + "]");
    });
}
