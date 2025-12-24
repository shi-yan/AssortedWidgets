// Uniforms for screen transformation
struct Uniforms {
    screen_size: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// Per-instance data (uploaded via instance buffer)
struct RectInstance {
    @location(0) rect: vec4<f32>,           // x, y, width, height
    @location(1) corner_radius: vec4<f32>,  // top_left, top_right, bottom_right, bottom_left
    @location(2) fill_color: vec4<f32>,     // rgba
    @location(3) border_color: vec4<f32>,   // rgba
    @location(4) border_width: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) local_pos: vec2<f32>,      // Position within rect (0,0 = center)
    @location(1) size: vec2<f32>,           // Half-size of rect
    @location(2) corner_radius: vec4<f32>,
    @location(3) fill_color: vec4<f32>,
    @location(4) border_color: vec4<f32>,
    @location(5) border_width: f32,
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

    // Convert to NDC (clip space)
    let ndc = (world_pos / uniforms.screen_size) * 2.0 - 1.0;
    out.clip_position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0); // Flip Y for screen coords

    // Local position (relative to rect center, in pixels)
    let center = instance.rect.xy + instance.rect.zw * 0.5;
    out.local_pos = world_pos - center;

    // Half-size for SDF calculation
    out.size = instance.rect.zw * 0.5;

    // Pass through style data
    out.corner_radius = instance.corner_radius;
    out.fill_color = instance.fill_color;
    out.border_color = instance.border_color;
    out.border_width = instance.border_width;

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

// Fragment shader: Render SDF with anti-aliasing
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate SDF distance
    let dist = sdf_rounded_box(in.local_pos, in.size, in.corner_radius);

    // Anti-aliased edge (smooth from -0.5 to +0.5 pixels)
    let fill_alpha = 1.0 - smoothstep(-0.5, 0.5, dist);

    // If no border, just return fill
    if (in.border_width <= 0.0) {
        return vec4<f32>(in.fill_color.rgb, in.fill_color.a * fill_alpha);
    }

    // Calculate border (two SDFs: outer and inner)
    let inner_dist = dist + in.border_width;
    let border_alpha = smoothstep(-0.5, 0.5, inner_dist) - smoothstep(-0.5, 0.5, dist);

    // Blend fill and border
    let fill_contribution = in.fill_color * (1.0 - border_alpha);
    let border_contribution = in.border_color * border_alpha;
    let final_color = fill_contribution + border_contribution;

    return vec4<f32>(final_color.rgb, final_color.a * fill_alpha);
}
