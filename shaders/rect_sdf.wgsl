// Uniforms for screen transformation
struct Uniforms {
    projection: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// Clip region uniform (max 8 nested clips)
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

// Per-instance data (uploaded via instance buffer)
struct RectInstance {
    @location(0) rect: vec4<f32>,           // x, y, width, height
    @location(1) corner_radius: vec4<f32>,  // top_left, top_right, bottom_right, bottom_left
    @location(2) fill_type: u32,            // 0 = solid, 1 = linear gradient, 2 = radial gradient
    @location(3) stop_count: u32,           // Number of gradient stops (2-8)
    @location(4) fill_color: vec4<f32>,     // rgba for solid color
    @location(5) border_color: vec4<f32>,   // rgba
    @location(6) border_width: f32,
    @location(7) gradient_start_end: vec4<f32>, // start.xy, end.xy (linear) or center.xy, radius, _
    @location(8) gradient_stop_0: vec4<f32>, // offset, r, g, b
    @location(9) gradient_stop_1: vec4<f32>,
    @location(10) gradient_stop_2: vec4<f32>,
    @location(11) gradient_stop_3: vec4<f32>,
    @location(12) gradient_stop_4: vec4<f32>,
    @location(13) gradient_stop_5: vec4<f32>,
    @location(14) gradient_stop_6: vec4<f32>,
    @location(15) gradient_stop_7: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) local_pos: vec2<f32>,      // Position within rect (0,0 = center)
    @location(1) size: vec2<f32>,           // Half-size of rect
    @location(2) corner_radius: vec4<f32>,
    @location(3) fill_type: u32,            // 0 = solid, 1 = linear gradient, 2 = radial gradient
    @location(4) stop_count: u32,           // Number of gradient stops (2-8)
    @location(5) fill_color: vec4<f32>,
    @location(6) border_color: vec4<f32>,
    @location(7) border_width: f32,
    @location(8) world_pos: vec2<f32>,      // World position for clipping
    @location(9) uv: vec2<f32>,             // UV coordinates (0-1) for gradient sampling
    @location(10) gradient_start_end: vec4<f32>,
    @location(11) gradient_stop_0: vec4<f32>,
    @location(12) gradient_stop_1: vec4<f32>,
    @location(13) gradient_stop_2: vec4<f32>,
    @location(14) gradient_stop_3: vec4<f32>,
    @location(15) gradient_stop_4: vec4<f32>,
    @location(16) gradient_stop_5: vec4<f32>,
    @location(17) gradient_stop_6: vec4<f32>,
    @location(18) gradient_stop_7: vec4<f32>,
}

// Vertex shader: Generate quad from instance data
@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: RectInstance,
) -> VertexOutput {
    var out: VertexOutput;

    // Quad vertices: 0=TL, 1=TR, 2=BL, 3=BR (triangle strip)
    let quad_pos = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0), // Top-left
        vec2<f32>(1.0, 0.0), // Top-right
        vec2<f32>(0.0, 1.0), // Bottom-left
        vec2<f32>(0.0, 1.0), // Bottom-left
        vec2<f32>(1.0, 0.0), // Top-right
        vec2<f32>(1.0, 1.0), // Bottom-right
    );

    let pos = quad_pos[vertex_index];

    // Calculate world position
    let world_pos = instance.rect.xy + pos * instance.rect.zw;

    // Convert to clip space using projection matrix
    out.clip_position = uniforms.projection * vec4<f32>(world_pos, 0.0, 1.0);

    // Local position (relative to rect center, in pixels)
    let center = instance.rect.xy + instance.rect.zw * 0.5;
    out.local_pos = world_pos - center;

    // Half-size for SDF calculation
    out.size = instance.rect.zw * 0.5;

    // Pass through style data
    out.corner_radius = instance.corner_radius;
    out.fill_type = instance.fill_type;
    out.stop_count = instance.stop_count;
    out.fill_color = instance.fill_color;
    out.border_color = instance.border_color;
    out.border_width = instance.border_width;
    out.world_pos = world_pos;
    out.uv = pos; // UV coordinates (0-1) for gradient sampling

    // Pass through gradient data
    out.gradient_start_end = instance.gradient_start_end;
    out.gradient_stop_0 = instance.gradient_stop_0;
    out.gradient_stop_1 = instance.gradient_stop_1;
    out.gradient_stop_2 = instance.gradient_stop_2;
    out.gradient_stop_3 = instance.gradient_stop_3;
    out.gradient_stop_4 = instance.gradient_stop_4;
    out.gradient_stop_5 = instance.gradient_stop_5;
    out.gradient_stop_6 = instance.gradient_stop_6;
    out.gradient_stop_7 = instance.gradient_stop_7;

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

// Sample a linear gradient at UV coordinate
fn sample_linear_gradient(
    uv: vec2<f32>,
    start_end: vec4<f32>,
    stops: array<vec4<f32>, 8>,
    stop_count: u32,
) -> vec4<f32> {
    let start = start_end.xy;
    let end = start_end.zw;

    // Calculate t along gradient direction
    let dir = end - start;
    let t = clamp(dot(uv - start, dir) / dot(dir, dir), 0.0, 1.0);

    // Find surrounding stops and interpolate
    for (var i = 0u; i < stop_count - 1u; i++) {
        let stop_a = stops[i];
        let stop_b = stops[i + 1u];

        if (t >= stop_a.x && t <= stop_b.x) {
            let local_t = (t - stop_a.x) / (stop_b.x - stop_a.x);
            let color_a = vec4<f32>(stop_a.yzw, 1.0);
            let color_b = vec4<f32>(stop_b.yzw, 1.0);
            return mix(color_a, color_b, local_t);
        }
    }

    // Return last color if we're past all stops
    let last_stop = stops[stop_count - 1u];
    return vec4<f32>(last_stop.yzw, 1.0);
}

// Sample a radial gradient at UV coordinate
fn sample_radial_gradient(
    uv: vec2<f32>,
    center_radius: vec4<f32>,
    stops: array<vec4<f32>, 8>,
    stop_count: u32,
) -> vec4<f32> {
    let center = center_radius.xy;
    let radius = center_radius.z;

    // Calculate distance from center normalized by radius
    let dist = length(uv - center) / radius;
    let t = clamp(dist, 0.0, 1.0);

    // Find surrounding stops and interpolate
    for (var i = 0u; i < stop_count - 1u; i++) {
        let stop_a = stops[i];
        let stop_b = stops[i + 1u];

        if (t >= stop_a.x && t <= stop_b.x) {
            let local_t = (t - stop_a.x) / (stop_b.x - stop_a.x);
            let color_a = vec4<f32>(stop_a.yzw, 1.0);
            let color_b = vec4<f32>(stop_b.yzw, 1.0);
            return mix(color_a, color_b, local_t);
        }
    }

    // Return last color if we're past all stops
    let last_stop = stops[stop_count - 1u];
    return vec4<f32>(last_stop.yzw, 1.0);
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

// Fragment shader: Render SDF with anti-aliasing and gradient support
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Apply clipping (discard if outside any clip region)
    if (apply_clipping(in.world_pos)) {
        discard;
    }

    // Calculate SDF distance
    let dist = sdf_rounded_box(in.local_pos, in.size, in.corner_radius);

    // Anti-aliased edge (smooth from -0.5 to +0.5 pixels)
    let fill_alpha = 1.0 - smoothstep(-0.5, 0.5, dist);

    // Determine fill color based on fill_type
    var fill_color: vec4<f32>;

    if (in.fill_type == 0u) {
        // Solid color
        fill_color = in.fill_color;
    } else if (in.fill_type == 1u) {
        // Linear gradient
        let stops = array<vec4<f32>, 8>(
            in.gradient_stop_0,
            in.gradient_stop_1,
            in.gradient_stop_2,
            in.gradient_stop_3,
            in.gradient_stop_4,
            in.gradient_stop_5,
            in.gradient_stop_6,
            in.gradient_stop_7,
        );
        fill_color = sample_linear_gradient(in.uv, in.gradient_start_end, stops, in.stop_count);
    } else {
        // Radial gradient (fill_type == 2u)
        let stops = array<vec4<f32>, 8>(
            in.gradient_stop_0,
            in.gradient_stop_1,
            in.gradient_stop_2,
            in.gradient_stop_3,
            in.gradient_stop_4,
            in.gradient_stop_5,
            in.gradient_stop_6,
            in.gradient_stop_7,
        );
        fill_color = sample_radial_gradient(in.uv, in.gradient_start_end, stops, in.stop_count);
    }

    // If no border, just return fill
    if (in.border_width <= 0.0) {
        return vec4<f32>(fill_color.rgb, fill_color.a * fill_alpha);
    }

    // Calculate border (two SDFs: outer and inner)
    let inner_dist = dist + in.border_width;
    let border_alpha = smoothstep(-0.5, 0.5, inner_dist) - smoothstep(-0.5, 0.5, dist);

    // Blend fill and border
    let fill_contribution = fill_color * (1.0 - border_alpha);
    let border_contribution = in.border_color * border_alpha;
    let final_color = fill_contribution + border_contribution;

    return vec4<f32>(final_color.rgb, final_color.a * fill_alpha);
}
