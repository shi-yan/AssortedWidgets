// Vertex shader for rendering colored rectangles with clipping support

struct VertexInput {
    @builtin(vertex_index) vertex_idx: u32,
    @location(0) rect: vec4<f32>,      // x, y, width, height
    @location(1) color: vec4<f32>,     // r, g, b, a
    @location(2) clip_rect: vec4<f32>, // clip x, y, width, height
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) world_pos: vec2<f32>,  // World-space position for clipping
    @location(2) clip_rect: vec4<f32>,  // Pass through clip rect
}

struct Uniforms {
    projection: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    // Generate quad from vertex index (0, 1, 2, 3)
    let positions = array<vec2<f32>, 4>(
        vec2(0.0, 0.0),  // Top-left
        vec2(1.0, 0.0),  // Top-right
        vec2(0.0, 1.0),  // Bottom-left
        vec2(1.0, 1.0),  // Bottom-right
    );

    let local_pos = positions[in.vertex_idx];

    // Transform to world space
    let world_pos = vec2(
        in.rect.x + local_pos.x * in.rect.z,
        in.rect.y + local_pos.y * in.rect.w,
    );

    // Convert to clip space using projection matrix
    let clip_pos = uniforms.projection * vec4<f32>(world_pos, 0.0, 1.0);

    var out: VertexOutput;
    out.position = clip_pos;
    out.color = in.color;
    out.world_pos = world_pos;
    out.clip_rect = in.clip_rect;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Clip test: discard pixels outside the clip rectangle
    let clip_min = in.clip_rect.xy;
    let clip_max = in.clip_rect.xy + in.clip_rect.zw;

    if (in.world_pos.x < clip_min.x || in.world_pos.x > clip_max.x ||
        in.world_pos.y < clip_min.y || in.world_pos.y > clip_max.y) {
        //discard;
        return in.color;
    }

    return in.color;
}
