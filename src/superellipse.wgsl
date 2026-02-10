#import bevy_ui::ui_vertex_output::UiVertexOutput

struct SuperellipseUiMaterial {
    /// Background color
    @location(0) background_color: vec4<f32>,
    /// Border color
    @location(1) border_color: vec4<f32>,
    /// border-radius of each corner:
    // (bottom-right, top-right, bottom-left, top-left)
    @location(2) border_radius: vec4<f32>,
    /// Border thickness: ignored if `border_color.a == 0.0`
    @location(3) border_thickness: f32,
    /// Inverse scale factor: must be updated to match the ComputedNode
    @location(4) inverse_scale_factor: f32,

    @location(5) time: f32,

    @location(6) turbulence_color: vec4<f32>,
    @location(7) power: f32,
    @location(8) resolution: vec2<f32>,
    @location(9) value: f32,
}

@group(1) @binding(0)
var<uniform> input: SuperellipseUiMaterial;

/// Adapted from:
///   https://www.shadertoy.com/view/4cG3R1
///
/// Related article:
///   https://iquilezles.org/articles/roundedboxes/
fn approx_sd_super_ellipse(p: vec2f, b: vec2f, r: vec4f) -> f32 {
    // select corner radius
    var r_xy = select(r.zw, r.xy, p.x > 0.);
    var n = select(r_xy.y, r_xy.x, p.y > 0.);

    let abs_p = abs(p);

    // really bad, cheap linearliation of the basic implicit formula
    n = 2.0 / n;
    let w = pow(abs_p.x / b.x, n) + pow(abs_p.y / b.y, n);
    let kb = 2.0 * n - 2.0;
    let ka = 1.0 - 1.0 / n;
    let kc = 2.0 * n;
    return (w - pow(w, ka)) * inverseSqrt(pow(abs_p.x, kb) / pow(b.x, kc) + pow(abs_p.y, kb) / pow(b.y, kc));
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // compute whether we should display the border
    let is_border = input.border_thickness > 0. && input.border_color.a > 0.;

    // adjust size by subtracting the border thickness
    var size = in.size;
    if is_border {
        size -= vec2f(input.border_thickness / input.inverse_scale_factor);
    }

    // adjust UVs around the middle of the rect, and convert to pixel
    // coordinates.
    let uv = in.uv * in.size * 2.0 - in.size;

    // define the shortest length of the image, as we need it to adjust the UV,
    // size and border coordinates.
    let min_size = min(in.size.x, in.size.y);

    // IMPORTANT: Minimum border radius of 0.2, otherwise the approximation
    // behaves strangely.
    let border_radius = max(input.border_radius / input.inverse_scale_factor / min_size, vec4f(0.2));

    // Compute signed distance
    let d = approx_sd_super_ellipse(
        uv / vec2f(min_size),
        size / vec2f(min_size),
        border_radius,
    );

    // define the alpha and color values depending on the distance sign.
    let alpha = select(input.background_color.a, 0., d > 0.);
    var col = select(
        //input.background_color.rgb,
        turbulence(in, input.turbulence_color, input.background_color),
        vec4f(0.),
        d > 0.0
    );

    // // Debug: Show distance
    // col *= 1.0 - exp(-6.0 * abs(d));
    // col *= 0.8 + 0.2 * cos(150.0 * d);

    // Apply border color
    var result = vec4f(col.rgb, alpha);
    if is_border {
        let border_thickness_uv = input.border_thickness / input.inverse_scale_factor / min_size;
        result = mix(result, input.border_color, 1.0 - smoothstep(0.0, border_thickness_uv, abs(d)));
    }

    return result;
}

fn rgba_shift(color: vec4<f32>) -> vec4<f32> {
    let shift = color.a - min(color.r, min(color.g, color.b)) - max(color.r, max(color.g, color.b));
    return vec4(shift + color.r, shift + color.g, shift + color.b, color.a);
}

const TAU = 6.28318530718;
const MAX_ITER = 5;

fn turbulence(in: UiVertexOutput, turbulence_color: vec4<f32>, base_color: vec4<f32>) -> vec4<f32> {

    let time = input.time * 0.5 + 23.0;
    // uv should be the 0-1 uv of texture...
    //var uv = in.uv;
    //var uv = in.uv + vec2(in.position.x / 1280.0, in.position.y / 720.0);
    //var uv = in.uv + in.position.xy / input.resolution.xy;
    var uv = in.uv / in.size.yx + in.position.xy / input.resolution.xy;

#ifdef SHOW_TILING
    let p = ((uv * TAU * 2.0) % TAU) - 250.0;
#else
    let p = ((uv * TAU) % TAU) - 250.0;
#endif
    var i = vec2(p);
    var c = 1.0;
    let inten = 0.005;

    for (var n = 0; n < MAX_ITER; n++) {
        let t = time * (1.0 - (3.5 / (f32(n) + 1.0)));
        i = p + vec2(cos(t - i.x) + sin(t + i.y), sin(t - i.y) + cos(t + i.x));
        c += 1.0 / length(vec2(p.x / (sin(i.x + t) / inten), p.y / (cos(i.y + t) / inten)));
    }
    c = c / f32(MAX_ITER);
    c = 1.17 - pow(c, 1.4);
    var color = vec3(pow(abs(c), 8.0));
    color = color * input.power;
    let t_color = color * turbulence_color.rgb;
    color = rgba_shift(vec4(color, 1.0)).rgb;
    color = mix(color * base_color.rgb, t_color, 0.5);

#ifdef SHOW_TILING
    // Flash tile borders...
    let pixel = 2.0 / vec2(input.resolution.x, input.resolution.y);
    uv *= 2.0;
    let f = floor(((input.time * 0.5) % 2.0));     // Flash value.
    let first = step(pixel, uv) * f;            // Rule out first screen pixels and flash.
    uv = step(fract(uv), pixel);                // Add one line of pixels per tile.
    color = mix(color, vec3(1.0, 1.0, 0.0), (uv.x + uv.y) * first.x * first.y); // Yellow line
#endif

    return vec4(color, base_color.a);
}
