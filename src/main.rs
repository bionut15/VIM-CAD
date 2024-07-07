use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_camera, spawn_plane, spawn_center, spawn_light),
        )
        .run();
}

//struct Coord {
//    x: f32,
//    y: f32,
//    z: f32,
//}

// need some rethinking about how
fn spawn_camera(mut commands: Commands) {
    let x_c: f32 = -2.5;
    let y_c: f32 = 5.0;
    let z_c: f32 = 2.5;

    let camera = Camera3dBundle {
        transform: Transform::from_xyz(x_c, y_c, z_c).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    };
    commands.spawn(camera);
}

// to do implement the color to a variable
fn spawn_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_scale = Vec3::new(1.0, 1.0, 1.0);
    let plane_xy = PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(1.5, 1.5))),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: plane_scale,
            ..default()
        },

        material: materials.add(Color::rgba(184.0, 210.0, 240.0, 0.29)),
        ..default()
    };
    let plane_xz = PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(1.5, 1.5))),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0), // center of the plane
            rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
            scale: plane_scale,
        },
        material: materials.add(Color::rgba(184.0, 210.0, 240.0, 0.29)),
        ..default()
    };
    let plane_yz = PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(1.5, 1.5))),
        material: materials.add(Color::rgba(184.0, 210.0, 240.0, 0.29)),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0), // center of the plane
            rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            scale: plane_scale,
            ..default()
        },
        ..default()
    };

    commands.spawn(plane_xy);
    commands.spawn(plane_xz);
    commands.spawn(plane_yz);
}
fn spawn_center(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let centerpoint = PbrBundle {
        mesh: meshes.add(Sphere::new(0.025).mesh().ico(7).unwrap()),
        material: materials.add(Color::BLACK),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    };
    commands.spawn(centerpoint);
}
fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0 * 1000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };

    commands.spawn(light);
}
