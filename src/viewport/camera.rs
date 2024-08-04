use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, TouchControls};
use std::f32::consts::TAU;

pub fn spawn_camera(mut commands: Commands) {
    let x_c: f32 = -2.5;
    let y_c: f32 = 5.0;
    let z_c: f32 = 2.5;

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(x_c, y_c, z_c))
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        // needs more fine tunning , works
        PanOrbitCamera {
            // Set focal point (what the camera should look at)
            focus: Vec3::new(0.0, 1.0, 0.0),
            // Set the starting position, relative to focus (overrides camera's transform).
            yaw: Some(TAU / 8.0),
            pitch: Some(TAU / 8.0),
            radius: Some(5.0),
            // Set limits on rotation and zoom
            yaw_upper_limit: Some(TAU / 4.0),
            yaw_lower_limit: Some(-TAU / 4.0),
            pitch_upper_limit: Some(TAU / 3.0),
            pitch_lower_limit: Some(-TAU / 3.0),
            zoom_upper_limit: Some(20.0),
            zoom_lower_limit: Some(1.0),
            // Adjust sensitivity of controls
            orbit_sensitivity: 1.5,
            pan_sensitivity: 0.5,
            zoom_sensitivity: 0.5,
            // Allow the camera to go upside down
            allow_upside_down: true,
            // Change the controls (these match Blender)
            button_orbit: MouseButton::Middle,
            button_pan: MouseButton::Middle,
            modifier_pan: Some(KeyCode::ShiftLeft),
            // Reverse the zoom direction
            reversed_zoom: false,
            // Use alternate touch controls
            touch_controls: TouchControls::TwoFingerOrbit,
            ..default()
        },
    ));
}

pub fn camera_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in query.iter_mut() {
        let rotation_speed = std::f32::consts::PI / 180.0;

        let pivot = Vec3::ZERO;

        let offset = transform.translation - pivot;

        if keyboard_input.pressed(KeyCode::KeyH) {
            let rotation = Quat::from_rotation_y(rotation_speed);
            transform.translation = pivot + rotation * offset;
            transform.look_at(pivot, Vec3::Y);
        }
        //need to rotate around an imaginary axis
        if keyboard_input.pressed(KeyCode::KeyJ) {
            let rotation = Quat::from_rotation_x(-rotation_speed);
            transform.translation = pivot + rotation * offset;
            transform.look_at(pivot, Vec3::Y);
        }
        if keyboard_input.pressed(KeyCode::KeyK) {
            let rotation = Quat::from_rotation_x(rotation_speed);
            transform.translation = pivot + rotation * offset;
            transform.look_at(pivot, Vec3::Y);
        }
        if keyboard_input.pressed(KeyCode::KeyL) {
            let rotation = Quat::from_rotation_y(-rotation_speed);
            transform.translation = pivot + rotation * offset;
            transform.look_at(pivot, Vec3::Y);
        }
    }
}
