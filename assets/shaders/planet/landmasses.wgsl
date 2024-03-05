#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import planet::common::{pm_common, rand, noise, fbm, dither, rotate, spherify};
    
struct MaterialLandmasses {
    light_origin: vec2<f32>,
    dither_size: f32,
    light_border_1: f32,
    light_border_2: f32,
    land_cutoff: f32,
    color1: vec4<f32>,
    color2: vec4<f32>,
    color3: vec4<f32>,
    color4: vec4<f32>,
}

@group(2) @binding(1) var<uniform> pm_under: MaterialLandmasses;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = floor(in.uv * pm_common.pixels) / pm_common.pixels;

    let dith = dither(in.uv, uv);

    var d_light = distance(uv, vec2<f32>(pm_under.light_origin));

    let d_circle = distance(uv, vec2<f32>(0.5));

    let a = step(d_circle, 0.49999);

    uv = rotate(uv, pm_common.rotation);
    uv = spherify(uv);

    let base_fbm_uv = (uv) * pm_common.size + vec2<f32>(globals.time * pm_common.time_speed, 0.0);

    var fbm1 = fbm(base_fbm_uv);
    var fbm2 = fbm(base_fbm_uv - pm_under.light_origin*fbm1);
    var fbm3 = fbm(base_fbm_uv - pm_under.light_origin*1.5*fbm1);
    var fbm4 = fbm(base_fbm_uv - pm_under.light_origin*2.0*fbm1);

    if (d_light < pm_under.light_border_1) {
        fbm4 *= 0.9;
    }
    if (d_light > pm_under.light_border_1) {
        fbm2 *= 1.05;
		fbm3 *= 1.05;
		fbm4 *= 1.05;
    }
    if (d_light > pm_under.light_border_2) {
        fbm2 *= 1.3;
		fbm3 *= 1.4;
		fbm4 *= 1.8;
    }

    d_light = pow(d_light, 2.0) * 0.1;
    var col = vec4<f32>(pm_under.color4);

    if (fbm4 + d_light < fbm1) {
    	col = col3;
	}
	if (fbm3 + d_light < fbm1) {
		col = col2;
	}
	if (fbm2 + d_light < fbm1) {
		col = col1;
	}

    return vec4<f32>(col.rgb, step(pm_under.land_cutoff, fbm1) * a * col.a);
}
