use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod camera;
mod canvas;
mod gui;

use camera::camera_movement_system;
use camera::spawn_camera;
use canvas::spawn_center;
use canvas::spawn_light;
use canvas::spawn_plane;
use gui::UI;

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
