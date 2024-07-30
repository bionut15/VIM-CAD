use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod app;
mod viewport;

use app::gui::UI;
use viewport::camera::camera_movement_system;
use viewport::camera::spawn_camera;
use viewport::canvas::spawn_center;
use viewport::canvas::spawn_light;
use viewport::canvas::spawn_plane;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(
            Startup,
            (spawn_camera, spawn_plane, spawn_center, spawn_light),
        )
        .add_systems(Update, (camera_movement_system, UI))
        .run();
}
