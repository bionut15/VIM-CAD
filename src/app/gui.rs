use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub fn UI(mut contexts: EguiContexts) {
    //egui::SidePanel::left("my_left_panel").show(contexts.ctx_mut(), |ui| {
    //    ui.label("Files/tree here");
    //});
    //implement  state machine like in video
    //make UI bar down there
    egui::TopBottomPanel::bottom("RustLine").show(contexts.ctx_mut(), |ui| {
        ui.label("[Normal]");
    });
}
