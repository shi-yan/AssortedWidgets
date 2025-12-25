// Simple Triangle Shader - Minimal test for RawSurface architecture

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;

    // Hardcoded triangle vertices in NDC space
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),   // Top
        vec2<f32>(-0.5, -0.5), // Bottom-left
        vec2<f32>(0.5, -0.5),  // Bottom-right
    );

    // RGB colors for each vertex
    var colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0), // Red
        vec3<f32>(0.0, 1.0, 0.0), // Green
        vec3<f32>(0.0, 0.0, 1.0), // Blue
    );

    output.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    output.color = colors[vertex_index];

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
