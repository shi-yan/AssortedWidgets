// Text rendering shader with support for monochrome and color glyphs
//
// Features:
// - Instanced quad rendering (4 vertices generated per glyph)
// - Texture array sampling (multi-page atlas)
// - Monochrome text (use alpha channel, apply color)
// - Color emoji (use RGB directly)
// - Shader-based clipping

struct Uniforms {
    screen_size: vec2<f32>,
    _padding: vec2<f32>,
}

struct VertexInput {
    @builtin(vertex_index) vertex_idx: u32,
    @location(0) position: vec2<f32>,
    @location(1) glyph_size: vec2<f32>,
    @location(2) uv_min: vec2<f32>,
    @location(3) uv_max: vec2<f32>,
    @location(4) color: vec4<f32>,
    @location(5) page_index: u32,
    @location(6) glyph_type: u32,
    @location(7) clip_rect: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) @interpolate(flat) page_index: u32,
    @location(3) @interpolate(flat) glyph_type: u32,
    @location(4) world_pos: vec2<f32>,
    @location(5) clip_rect: vec4<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(0) var atlas: texture_2d_array<f32>;
@group(1) @binding(1) var atlas_sampler: sampler;

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
    let world_pos = in.position + corner * in.glyph_size;

    // Convert to clip space (-1 to 1)
    let clip_pos = vec2<f32>(
        (world_pos.x / uniforms.screen_size.x) * 2.0 - 1.0,
        1.0 - (world_pos.y / uniforms.screen_size.y) * 2.0,  // Y-flip
    );

    // Interpolate UV coordinates
    let uv = mix(in.uv_min, in.uv_max, corner);

    var out: VertexOutput;
    out.position = vec4<f32>(clip_pos, 0.0, 1.0);
    out.uv = uv;
    out.color = in.color;
    out.page_index = in.page_index;
    out.glyph_type = in.glyph_type;
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

    // Sample from texture array (layer = page_index)
    let sampled = textureSample(atlas, atlas_sampler, in.uv, in.page_index);

    // Different handling for monochrome vs color glyphs
    if (in.glyph_type == 0u) {
        // Monochrome text: use alpha channel, apply color
        // The glyph mask is in the alpha channel
        return vec4<f32>(in.color.rgb, in.color.a * sampled.a);
    } else {
        // Color emoji: use RGB directly, multiply alpha
        return vec4<f32>(sampled.rgb, sampled.a * in.color.a);
    }
}
