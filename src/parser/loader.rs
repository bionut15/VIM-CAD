//add shader functionality
//use crate::shaders::shader;
use bevy::prelude::*;
use std::env;

// add func to read from console with
// make it read from a config file
pub fn load_model(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("models/test.gltf"),
    );
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(1.8, 2.7, 8.6),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: cube_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

pub fn ReadCLI() {
    // add that args for instead of load model
    let args: Vec<String> = env::args().collect();
}
