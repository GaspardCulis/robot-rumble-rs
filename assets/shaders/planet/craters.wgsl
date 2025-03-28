#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import planet::common::{pm_common, rand, noise, fbm, dither, rotate, spherify, circleNoise};
    
struct CratersMaterial {
    light_border: f32,
    color1: vec4<f32>,
    color2: vec4<f32>,
}

@group(2) @binding(1) var<uniform> pm_craters: CratersMaterial;


fn crater(uv: vec2<f32>) -> f32 {
	var c: f32 = 1.0;
	for (var i:f32 = 0; i < 2; i+=1.0) {
		c *= circleNoise((uv * pm_common.size) + (i+11.) + vec2<f32>(globals.time*pm_common.time_speed, 0.0));
	}
	return 1.0 - c;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = floor(in.uv * pm_common.pixels) / pm_common.pixels;

    var d_light = distance(uv, pm_common.light_origin);
    let d_circle = distance(uv, vec2<f32>(0.5));

    var a = step(d_circle, 0.49999);

    uv = spherify(uv);
    uv = rotate(uv, pm_common.rotation);

    let c1 = crater(uv );
	let c2 = crater(uv +(pm_common.light_origin - 0.5)*0.03);
	var col = pm_craters.color1;

    a *= step(0.5, c1);
	if (c2 < c1 - (0.5 - d_light)*2.0) {
		col = pm_craters.color2;
	}
	if (d_light > pm_craters.light_border) {
		col = pm_craters.color2;
	} 

	a *= step(d_circle, 0.5);
    
    return vec4<f32>(col.rgb, a * col.a);
}
