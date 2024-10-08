[[stage(fragment)]]
fn fragment_main(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    // Edge detection logic
    let edge_color: vec4<f32> = vec4<f32>(1.0, 0.0, 0.0, 1.0); // Red color for edges
    let base_color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0); // Transparent base color

    let edge_threshold: f32 = 0.1;
    if (length(input.normal) < edge_threshold) {
        return edge_color;
    } else {
        return base_color;
    }
}

#import bevy_pbr::forward_io::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
#import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var material_color_texture: texture_2d<f32>;
@group(2) @binding(2) var material_color_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return material_color * textureSample(material_color_texture, material_color_sampler, mesh.uv) * COLOR_MULTIPLIER;
}

