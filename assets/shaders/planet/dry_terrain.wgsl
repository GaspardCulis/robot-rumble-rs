#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import planet::common::{pm_common, rand, noise, fbm, dither, rotate, spherify};

struct DryTerrainMaterial {
    dither_size: f32,
    light_distance_1: f32,
    light_distance_2: f32,
    colors: texture_2d<f32>,
    colors_sampler: sampler
}

@group(2) @binding(1) var<uniform> pm_dry: DryTerrainMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = floor(in.uv * pm_common.pixels) / pm_common.pixels;
    let dith = dither(in.uv, uv);
    
    let d_circle = distance(uv, vec2<f32>(0.5));
    let a = step(d_circle, 0.49999);

    uv = spheriphy(uv, pm_common.rotation);

    var d_light = distance(uv, pm_common.light_origin);

    uv = rotate(uv, pm_common.rotation);

    var f = fbm(uv * pm_common.size + vec2<f32>(globals.time * pm_common.time_speed, 0.0));

    d_light = smoothstep(-0.3, 1.2, d_light);

    if (d_light < pm_dry.light_distance_1) {
		d_light *= 0.9;
	}
	if (d_light < pm_dry.light_distance_2) {
		d_light *= 0.9;
	}

    var c = d_light*pow(f,0.8)*3.5;

    if dith {
        c += 0.02;
		c *= 1.05;
    }

    var posterize = floor(c*4.0)/4.0;

    var col = textureSample(colors, vec2(posterize, 0.0));
    
    return vec4<f32>(col.rgb, a * col.a);
}
