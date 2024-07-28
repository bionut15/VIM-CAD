use bevy::prelude::*;

mod camera;
mod canvas;

use camera::camera_movement_system;
use camera::spawn_camera;
use canvas::spawn_center;
use canvas::spawn_light;
use canvas::spawn_plane;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_camera, spawn_plane, spawn_center, spawn_light),
        )
        .add_systems(Update, camera_movement_system)
        .run();
}
