use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

mod app;
mod parser;
mod shaders;
mod viewport;

use app::gui::UI;
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
