#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import planet::common::{pm_common, dither, fbm, rand, rotate, spherify};

struct StarFlaresMaterial {
    scale: f32,
    storm_width: f32,
    storm_dither_width: f32,
    circle_amount: f32,
    circle_scale: f32,
}

@group(2) @binding(1) var<uniform> pm_star_flares: StarFlaresMaterial;

@group(2) @binding(2) var material_colorscheme_texture: texture_2d<f32>;
@group(2) @binding(3) var material_colorscheme_sampler: sampler;

fn circle(p_uv: vec2<f32>) -> f32 {
    var uv = p_uv;
    let invert = 1.0 / pm_star_flares.circle_amount;

    if uv.y % (invert * 2.0) < invert {
        uv.x += invert * 0.5;
    }
    let rand_co = floor(uv * pm_star_flares.circle_amount) / pm_star_flares.circle_amount;
    uv = (uv % invert) * pm_star_flares.circle_amount;

    var r = rand(rand_co);
    r = clamp(r, invert, 1.0 - invert);
    let circle = distance(uv, vec2<f32>(r, r));
    return smoothstep(circle, circle + 0.5, invert * pm_star_flares.circle_scale * rand(rand_co * 1.5));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = floor(in.uv * pm_common.pixels) / pm_common.pixels;

    let dith = dither(in.uv, uv);

    uv = rotate(uv, pm_common.rotation);

    let angle = atan2(uv.x - 0.5, uv.y - 0.5) * 0.4;

    let d = distance(uv, vec2<f32>(0.5));

    let circleUV = vec2<f32>(d, angle);

    let n = fbm(circleUV * pm_common.size - globals.time * pm_common.time_speed);
    var nc = circle(circleUV * pm_star_flares.scale - globals.time * pm_common.time_speed + n);

    nc *= 1.5;
    let n2 = fbm(circleUV * pm_common.size - globals.time + vec2<f32>(100.0, 100.0));
    nc -= n2 * 0.1;

	// our alpha, default 0
    var a = 0.0;
    if 1.0 - d > nc {
		// now we generate very thin strips of positive alpha if our noise has certain values and is close enough to center
        if nc > pm_star_flares.storm_width - pm_star_flares.storm_dither_width + d && dith {
            a = 1.0;
        } else if nc > pm_star_flares.storm_width + d { // could use an or statement instead, but this looks more clear to me
            a = 1.0;
		}
    }

	// use our two noise values to assign colors
    let interpolate = floor(n2 + nc);
    let col = textureSample(material_colorscheme_texture, material_colorscheme_sampler, vec2<f32>(interpolate, 0.0));

	// final step to not have everything appear from the center
    a *= step(n2 * 0.25, d);
    return vec4<f32>(col.rgb, a * col.a);
}
