#import bevy_sprite::mesh2d_vertex_output::VertexOutput;

struct BackgroundStarsMaterial {
    size: f32,
    octaves: i32,
    seed: f32,
    pixels: f32,
    uv_correct: vec2<f32>,
    _wasm_padding: vec2<f32>,
}

@group(2) @binding(0) var<uniform> bg_stars: BackgroundStarsMaterial;

@group(2) @binding(1) var material_colorscheme_texture: texture_2d<f32>;
@group(2) @binding(2) var material_colorscheme_sampler: sampler;

fn rand(p_coord: vec2<f32>, tilesize: f32) -> f32 {
    var coord = p_coord;

	return fract(sin(dot(coord, vec2<f32>(12.9898,78.233))) * (15.5453 + bg_stars.seed));
}

fn noise(coord: vec2<f32>, tilesize: f32) -> f32 {
	let i = floor(coord);
	let f = fract(coord);
		
	let a = rand(i, tilesize);
	let b = rand(i + vec2(1.0, 0.0), tilesize);
	let c = rand(i + vec2(0.0, 1.0), tilesize);
	let d = rand(i + vec2(1.0, 1.0), tilesize);

	let cubic = f * f * (3.0 - 2.0 * f);

	return mix(a, b, cubic.x) + (c - a) * cubic.y * (1.0 - cubic.x) + (d - b) * cubic.x * cubic.y;
}

fn fbm(p_coord: vec2<f32>, tilesize: f32) -> f32 {
    var coord = p_coord;
	var value = 0.0;
	var scale = 0.5;

	for(var i = 0; i < bg_stars.octaves; i+=1){
		value += noise(coord, tilesize) * scale;
		coord *= 2.0;
		scale *= 0.5;
	}
	return value;
}

fn dither(uv1: vec2<f32>, uv2: vec2<f32>) -> bool {
    let tmp: f32 = (uv1.y + uv2.x) % (2.0 / bg_stars.pixels);
	return tmp <= 1.0 / bg_stars.pixels;
}

fn circleNoise(p_uv: vec2<f32>, tilesize: f32) -> f32 {
    var uv = p_uv;
	
    let uv_y = floor(uv.y);
    uv.x += uv_y*.31;
    let f = fract(uv);
	let h = rand(vec2<f32>(floor(uv.x),floor(uv_y)), tilesize);
    let m = (length(f-0.25-(h*0.5)));
    let r = h*0.25;
    return smoothstep(0.0, r, m*0.75);
}

fn rotate(p_vec: vec2<f32>, angle: f32) -> vec2<f32> {
	var vec = p_vec - vec2<f32>(0.5);
	vec *= mat2x2<f32>(vec2<f32>(cos(angle),-sin(angle)), vec2<f32>(sin(angle),cos(angle)));
	vec += vec2<f32>(0.5);
	return vec;
}

fn cloud_alpha(uv: vec2<f32>, tilesize: f32) -> f32 {
	var c_noise = 0.0;
	
	// more iterations for more turbulence
	let iters = 2;
	for (var i = 0; i < iters; i+=1){
		c_noise += circleNoise(uv * 0.5 + (f32(i+1)) + vec2<f32>(-0.3, 0.0), ceil(tilesize * 0.5));
	}
	let fbm = fbm(uv+c_noise, tilesize);
	
	return fbm;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // pixelizing and dithering
	let uv = floor((in.uv) * bg_stars.pixels) / bg_stars.pixels * bg_stars.uv_correct;
	let dith = dither(uv, in.uv);
	
	// noise for the dust
	// the + vec2(x,y) is to create an offset in noise values
	let n_alpha = fbm(uv * ceil(bg_stars.size * 0.5) + vec2<f32>(2,2), ceil(bg_stars.size * 0.5));
	let n_dust = cloud_alpha(uv * bg_stars.size, bg_stars.size);
	let n_dust2 = fbm(uv * ceil(bg_stars.size * 0.2)  -vec2<f32>(2,2),ceil(bg_stars.size * 0.2));
	var n_dust_lerp = n_dust2 * n_dust;

	// apply dithering
	if (dith) {
		n_dust_lerp *= 0.95;
	}

	// choose alpha value
	let a_dust = step(n_alpha , n_dust_lerp * 1.8);
	n_dust_lerp = pow(n_dust_lerp, 3.2) * 56.0;
	if (dith) {
		n_dust_lerp *= 1.1;
	}

	// bg_nebulae.reduce_background
	if (false) {
		n_dust_lerp = pow(n_dust_lerp, 0.8) * 0.7;
	}
	
	let col_value = floor(n_dust_lerp) / 7.0;
	let col = textureSample(material_colorscheme_texture, material_colorscheme_sampler, vec2<f32>(col_value, 0.0));
	
	
	
	return vec4<f32>(col.rgb, a_dust);
}
