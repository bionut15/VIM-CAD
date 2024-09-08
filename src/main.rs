use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

//if u use modules outside of main u have to declare them here xd
mod app {
    pub mod gui;
    pub mod keymaps;
    pub mod state;
}
mod parser;
mod shaders;
mod viewport;

use crate::app::state::{MainWindow, StateApp, Transition::*};
use app::{gui::UI, keymaps::keymaps::keyboard_input};
use parser::loader::load_model;
use viewport::camera::camera_movement_system;
use viewport::camera::spawn_camera;
use viewport::canvas::spawn_center;
use viewport::canvas::spawn_light;
use viewport::canvas::spawn_plane;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(
            Startup,
            (
                //change from load to ReadCLI func
                load_model,
                spawn_camera,
                spawn_plane,
                spawn_center,
                spawn_light,
            ),
        )
        .add_systems(Update, (camera_movement_system, UI))
        .run();
}
