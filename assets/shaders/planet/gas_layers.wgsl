#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import planet::common::{pm_common, rand, noise, fbm, dither, rotate, spherify};
    
struct GasLayersMaterial {
    bands: f32,
    stretch: f32,
    cloud_cover: f32,
    cloud_curve: f32,
    light_border_1: f32,
    light_border_2: f32,
}

@group(2) @binding(1) var<uniform> pm_under: GasLayersMaterial;

@group(2) @binding(2) var material_colorscheme_texture: texture_2d<f32>;
@group(2) @binding(3) var material_colorscheme_sampler: sampler;

@group(2) @binding(4) var material_darkcolorscheme_texture: texture_2d<f32>;
@group(2) @binding(5) var material_darkcolorscheme_sampler: sampler;

// Has slightly different config than the common definition
fn circleNoise(uv: vec2<f32>) -> f32 {
	let uv_y = floor(uv.y);
	var tmp_uv = uv;
    tmp_uv.x += uv_y*.31;
    let f = fract(tmp_uv);
	let h = rand(vec2<f32>(floor(tmp_uv.x),floor(uv_y)));
    let m = (length(f-0.25-(h*0.5)));
    let r = h*0.25;
    return smoothstep(0.0, r, m*0.75);
}

fn turbulence(uv: vec2<f32>) -> f32 {
	var c_noise = 0.0;
	
	// more iterations for more turbulence
	for (var i: f32 = 0.0; i < 10.0; i += 1.0) {
		c_noise += circleNoise((uv * pm_common.size *0.3) + (i + 11.) + (vec2<f32>(globals.time * pm_common.time_speed, 0.0)));
	}
	return c_noise;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = floor(in.uv * pm_common.pixels) / pm_common.pixels;

    let dith = dither(in.uv, uv);

    var d_light = distance(uv, pm_common.light_origin);

    let a = step(length(uv-vec2<f32>(0.5)), 0.49999);

    uv = rotate(uv, pm_common.rotation);
    uv = spherify(uv);

    let band = fbm(vec2<f32>(0.0, uv.y*pm_common.size*pm_under.bands));

    let turb = turbulence(uv);

    let fbm1 = fbm(uv*pm_common.size);
	var fbm2 = fbm(uv*vec2<f32>(1.0, 2.0)*pm_common.size+fbm1+vec2<f32>(-globals.time*pm_common.time_speed,0.0)+turb);

    fbm2 *= pow(band,2.0)*7.0;
	let light = fbm2 + d_light*1.8;
    fbm2 += pow(d_light, 1.0)-0.3;
	fbm2 = smoothstep(-0.2, 4.0-fbm2, light);

    if dith {
        fbm2 *= 1.1;
    }

    let posterized = floor(fbm2*4.0)/2.0;

    var col = vec4<f32>(0., 0., 0., 1.0);
    if fbm2 < 0.625 {
        col = textureSample(material_colorscheme_texture, material_colorscheme_sampler, vec2<f32>(posterized, uv.y));
    } else {
        col = textureSample(material_darkcolorscheme_texture, material_darkcolorscheme_sampler, vec2<f32>(posterized-1.0, uv.y));
    }

    return vec4<f32>(col.rgb, a * col.a);
}
