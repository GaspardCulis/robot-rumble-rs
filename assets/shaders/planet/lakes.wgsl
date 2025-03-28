#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import planet::common::{pm_common, rand, noise, fbm, dither, rotate, spherify};
    
struct LakesMaterial {
    light_border_1: f32,
    light_border_2: f32,
    lake_cutoff: f32,
    color1: vec4<f32>,
    color2: vec4<f32>,
    color3: vec4<f32>,
}

@group(2) @binding(1) var<uniform> pm_lakes: LakesMaterial;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = floor(in.uv * pm_common.pixels) / pm_common.pixels;

    var d_light = distance(uv, pm_common.light_origin);

    uv = rotate(uv, pm_common.rotation);

    var fbm1 = fbm(uv * pm_common.size + vec2<f32>(globals.time * pm_common.time_speed, 0.0));
    var lake = fbm(uv * pm_common.size + vec2<f32>(globals.time * pm_common.time_speed, 0.0));

    d_light = pow(d_light, 2.0)*0.4;
	d_light -= d_light * lake;

    var col = pm_lakes.color1;
	if (d_light > pm_lakes.light_border_1) {
		col = pm_lakes.color2;
	}
	if (d_light > pm_lakes.light_border_2) {
		col = pm_lakes.color3;
	}

    var a = step(pm_lakes.lake_cutoff, lake);
    a *= step(distance(vec2<f32>(0.5, 0.5), uv), 0.5);

    return vec4<f32>(col.rgb, a * col.a);
}
