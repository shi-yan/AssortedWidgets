// Shadow SDF Shader - Analytical soft shadows
//
// This shader renders drop shadows using an analytical approach:
// - Single pass (no multi-pass blur)
// - Soft, smooth gradients via SDF distance field
// - Configurable offset, blur radius, and spread

// Uniforms for screen transformation
struct Uniforms {
    screen_size: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// Clip region uniform (max 8 nested clips) - shared with rect shader
struct ClipRegion {
    rect: vec4<f32>,           // x, y, width, height
    corner_radius: vec4<f32>,  // top_left, top_right, bottom_right, bottom_left
}

struct ClipUniforms {
    count: u32,                 // Number of active clip regions (0-8)
    _padding: vec3<u32>,        // Align to 16 bytes
    regions: array<ClipRegion, 8>,
}

@group(1) @binding(0)
var<uniform> clip_uniforms: ClipUniforms;

// Per-instance shadow data (uploaded via instance buffer)
struct ShadowInstance {
    @location(0) rect: vec4<f32>,           // x, y, width, height (of the shape casting shadow)
    @location(1) corner_radius: vec4<f32>,  // top_left, top_right, bottom_right, bottom_left
    @location(2) shadow_color: vec4<f32>,   // rgba
    @location(3) offset: vec2<f32>,         // Shadow offset (x, y)
    @location(4) blur_radius: f32,          // Blur amount
    @location(5) spread_radius: f32,        // Expand/contract before blur
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) local_pos: vec2<f32>,      // Position within shadow quad (relative to center)
    @location(1) size: vec2<f32>,           // Half-size of shadow quad
    @location(2) corner_radius: vec4<f32>,
    @location(3) shadow_color: vec4<f32>,
    @location(4) blur_radius: f32,
    @location(5) spread_radius: f32,
    @location(6) world_pos: vec2<f32>,      // World position for clipping
}

// Vertex shader: Generate quad for shadow
@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: ShadowInstance,
) -> VertexOutput {
    var out: VertexOutput;

    // Quad vertices (6 vertices for 2 triangles)
    let quad_pos = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0), // Top-left
        vec2<f32>(1.0, 0.0), // Top-right
        vec2<f32>(0.0, 1.0), // Bottom-left
        vec2<f32>(0.0, 1.0), // Bottom-left
        vec2<f32>(1.0, 0.0), // Top-right
        vec2<f32>(1.0, 1.0), // Bottom-right
    );

    let pos = quad_pos[vertex_index];

    // Shadow quad extends beyond rect by blur_radius + spread_radius
    let expansion = instance.blur_radius * 2.5 + instance.spread_radius;
    let shadow_rect = vec4<f32>(
        instance.rect.x + instance.offset.x - expansion,
        instance.rect.y + instance.offset.y - expansion,
        instance.rect.z + expansion * 2.0,
        instance.rect.w + expansion * 2.0,
    );

    // Calculate world position
    let world_pos = shadow_rect.xy + pos * shadow_rect.zw;

    // Convert to NDC (clip space)
    let ndc = (world_pos / uniforms.screen_size) * 2.0 - 1.0;
    out.clip_position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0); // Flip Y for screen coords

    // Local position (relative to shadow center, in pixels)
    let center = shadow_rect.xy + shadow_rect.zw * 0.5;
    out.local_pos = world_pos - center;

    // Half-size for SDF calculation (of the ORIGINAL rect, not shadow quad)
    let original_center = instance.rect.xy + instance.rect.zw * 0.5 + instance.offset;
    out.size = instance.rect.zw * 0.5 + vec2<f32>(instance.spread_radius);

    // Adjust local_pos to be relative to original rect center (with offset)
    out.local_pos = world_pos - original_center;

    // Pass through shadow data
    out.corner_radius = instance.corner_radius;
    out.shadow_color = instance.shadow_color;
    out.blur_radius = instance.blur_radius;
    out.spread_radius = instance.spread_radius;
    out.world_pos = world_pos;

    return out;
}

// SDF for rounded box (returns signed distance)
fn sdf_rounded_box(p: vec2<f32>, size: vec2<f32>, radius: vec4<f32>) -> f32 {
    // Select corner radius based on quadrant
    var r = radius.x; // default to top-left
    if (p.x > 0.0) {
        if (p.y > 0.0) {
            r = radius.z; // bottom-right
        } else {
            r = radius.y; // top-right
        }
    } else {
        if (p.y > 0.0) {
            r = radius.w; // bottom-left
        }
    }

    // Clamp radius to rect size
    r = min(r, min(size.x, size.y));

    // SDF calculation
    let q = abs(p) - size + r;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0))) - r;
}

// Check if a point is inside all active clip regions (returns true if clipped OUT)
fn apply_clipping(world_pos: vec2<f32>) -> bool {
    for (var i = 0u; i < clip_uniforms.count; i++) {
        let region = clip_uniforms.regions[i];

        // Calculate position relative to clip region center
        let center = region.rect.xy + region.rect.zw * 0.5;
        let local_pos = world_pos - center;
        let half_size = region.rect.zw * 0.5;

        // Evaluate SDF for this clip region
        let dist = sdf_rounded_box(local_pos, half_size, region.corner_radius);

        // If outside this clip region, discard the fragment
        if (dist > 0.0) {
            return true; // Clipped out
        }
    }

    return false; // Inside all clips
}

// Fragment shader: Render analytical soft shadow
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Apply clipping (discard if outside any clip region)
    if (apply_clipping(in.world_pos)) {
        discard;
    }

    // Calculate SDF distance to shape
    let dist = sdf_rounded_box(in.local_pos, in.size, in.corner_radius);

    // Analytical shadow: smooth falloff based on distance from edge
    // - Inside shape (dist < 0): no shadow (fully transparent)
    // - At edge (dist = 0): full shadow opacity
    // - Outside (dist > 0): smooth falloff based on blur_radius

    // Smooth transition from 0 to blur_radius
    let shadow_alpha = 1.0 - smoothstep(0.0, in.blur_radius, dist);

    // Inside the shape, no shadow (we only want DROP shadows, not fills)
    let alpha = select(shadow_alpha, 0.0, dist < -0.5);

    return vec4<f32>(in.shadow_color.rgb, in.shadow_color.a * alpha);
}
