#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import planet::common::{pm_common, rand, noise, fbm, dither, rotate, spherify, circleNoise};

struct CloudsMaterial {
    cloud_cover: f32,
    stretch: f32,
    cloud_curve: f32,
    light_border_1: f32,
    light_border_2: f32,
    base_color: vec4<f32>,
    outline_color: vec4<f32>,
    shadow_color: vec4<f32>,
    shadow_outline_color: vec4<f32>,
}

fn cloud_alpha(uv: vec2<f32>) -> f32 {
	var c_noise: f32 = 0.0;
	
	// more iterations for more turbulence
	for (var i: f32 = 0; i < 9.; i+=1.0) {
		c_noise += circleNoise((uv * pm_common.size * 0.3) + (i+11.) + (vec2<f32>(globals.time*pm_common.time_speed, 0.0)));
	}
	let fbm = fbm(uv*pm_common.size+c_noise + vec2<f32>(globals.time*pm_common.time_speed, 0.0));
	
	return fbm;
}

@group(2) @binding(1) var<uniform> pm_clouds: CloudsMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
	var uv = floor(in.uv*pm_common.pixels)/pm_common.pixels;
	
	let d_light = distance(uv , pm_common.light_origin);
	
	let d_circle = distance(uv, vec2<f32>(0.5));
	let a = step(d_circle, 0.5);
	
	let d_to_center = distance(uv, vec2<f32>(0.5));
	
	uv = rotate(uv, pm_common.rotation);
	
	uv = spherify(uv);
	uv.y += smoothstep(0.0, pm_clouds.cloud_curve, abs(uv.x-0.4));
	
	
	var c = cloud_alpha(uv*vec2<f32>(1.0, pm_clouds.stretch));
	
	var col = pm_clouds.base_color;
	if (c < pm_clouds.cloud_cover + 0.03) {
		col = pm_clouds.outline_color;
	}
	if (d_light + c*0.2 > pm_clouds.light_border_1) {
		col = pm_clouds.shadow_color;

	}
	if (d_light + c*0.2 > pm_clouds.light_border_2) {
		col = pm_clouds.shadow_outline_color;
	}
	
	c *= step(d_to_center, 0.5);
	return vec4<f32>(col.rgb, step(pm_clouds.cloud_cover, c) * a * col.a);
}
