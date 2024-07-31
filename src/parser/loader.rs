use bevy::prelude::*;

pub fn load_model(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Load the GLTF model
    let my_gltf = asset_server.load("/models/test.glb#Scene0");

    // Spawn the GLTF scene
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0), // Adjust the scale as needed
            rotation: Quat::from_rotation_y(0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
