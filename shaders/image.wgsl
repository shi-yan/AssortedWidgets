// Image rendering shader for textured quads
//
// Features:
// - Simple texture sampling (individual textures, not atlas)
// - Optional color tinting
// - Shader-based clipping
// - Instanced rendering support

struct Uniforms {
    projection: mat4x4<f32>,
}

struct VertexInput {
    @builtin(vertex_index) vertex_idx: u32,
    @location(0) position: vec2<f32>,      // Top-left corner in screen space
    @location(1) size: vec2<f32>,          // Width and height
    @location(2) tint: vec4<f32>,          // Color tint (1,1,1,1 = no tint)
    @location(3) clip_rect: vec4<f32>,     // Clipping rectangle
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) tint: vec4<f32>,
    @location(2) world_pos: vec2<f32>,
    @location(3) clip_rect: vec4<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(0) var image_texture: texture_2d<f32>;
@group(1) @binding(1) var image_sampler: sampler;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    // Generate quad corners from vertex index (0, 1, 2, 3)
    // Using triangle strip: (0,0), (1,0), (0,1), (1,1)
    let corners = array<vec2<f32>, 4>(
        vec2<f32>(0.0, 0.0),  // Top-left
        vec2<f32>(1.0, 0.0),  // Top-right
        vec2<f32>(0.0, 1.0),  // Bottom-left
        vec2<f32>(1.0, 1.0),  // Bottom-right
    );
    let corner = corners[in.vertex_idx];

    // World-space position
    let world_pos = in.position + corner * in.size;

    // Convert to clip space using projection matrix
    let clip_pos = uniforms.projection * vec4<f32>(world_pos, 0.0, 1.0);

    // UV coordinates (standard 0-1 range)
    let uv = corner;

    var out: VertexOutput;
    out.position = clip_pos;
    out.uv = uv;
    out.tint = in.tint;
    out.world_pos = world_pos;
    out.clip_rect = in.clip_rect;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Clipping test: discard if outside clip rectangle
    let clip_min = in.clip_rect.xy;
    let clip_max = in.clip_rect.xy + in.clip_rect.zw;

    if (in.world_pos.x < clip_min.x || in.world_pos.x > clip_max.x ||
        in.world_pos.y < clip_min.y || in.world_pos.y > clip_max.y) {
        discard;
    }

    // Sample texture
    let sampled = textureSample(image_texture, image_sampler, in.uv);

    // Apply tint (multiply)
    return sampled * in.tint;
}
