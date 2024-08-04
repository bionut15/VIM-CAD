// edge_shader.wgsl

[[block]] struct Uniforms {
    model_view_projection_matrix: mat4x4<f32>;
};

[[group(0), binding(0)]] var<uniform> uniforms: Uniforms;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] normal: vec3<f32>;
};

[[stage(vertex)]]
fn vertex_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = uniforms.model_view_projection_matrix * vec4<f32>(input.position, 1.0);
    output.normal = input.normal;
    return output;
}

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

