use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    let x_c: f32 = -2.5;
    let y_c: f32 = 5.0;
    let z_c: f32 = 2.5;

    let camera = Camera3dBundle {
        transform: Transform::from_xyz(x_c, y_c, z_c).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    };
    commands.spawn(camera);
}

pub fn camera_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in query.iter_mut() {
        let rotation_speed = std::f32::consts::PI / 180.0; // Rotation speed in radians

        let pivot = Vec3::ZERO;

        let offset = transform.translation - pivot;

        if keyboard_input.pressed(KeyCode::KeyH) {
            let rotation = Quat::from_rotation_y(rotation_speed);
            transform.translation = pivot + rotation * offset;
            transform.look_at(pivot, Vec3::Y);
        }
        if keyboard_input.pressed(KeyCode::KeyL) {
            let rotation = Quat::from_rotation_y(-rotation_speed);
            transform.translation = pivot + rotation * offset;
            transform.look_at(pivot, Vec3::Y);
        }
        if keyboard_input.pressed(KeyCode::KeyJ) {
            let rotation = Quat::from_rotation_z(rotation_speed);
            transform.translation = pivot + rotation * offset;
            transform.look_at(pivot, Vec3::Y);
        }
        if keyboard_input.pressed(KeyCode::KeyK) {
            let rotation = Quat::from_rotation_z(-rotation_speed);
            transform.translation = pivot + rotation * offset;
            transform.look_at(pivot, Vec3::Y);
        }
    }
}
