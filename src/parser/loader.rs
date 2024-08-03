use bevy::gltf;
use bevy::prelude::*;

// make it read from a config file
const Dss: &str = "ssss";

pub fn load_model(commands: Commands, asset_server: Res<AssetServer>) {
    // Load the GLTF model
    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/pipe.gltf")),
        ..default()
    });
}
