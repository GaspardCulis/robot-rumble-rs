#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import planet::common::{pm_common, rand, noise, fbm, dither, rotate, spherify};
    
struct RingMaterial {
    ring_width: f32,
    ring_perspective: f32,
    scale_rel_to_planet: f32
}

@group(2) @binding(1) var<uniform> pm_ring: RingMaterial;

@group(2) @binding(2) var material_colorscheme_texture: texture_2d<f32>;
@group(2) @binding(3) var material_colorscheme_sampler: sampler;

@group(2) @binding(4) var material_darkcolorscheme_texture: texture_2d<f32>;
@group(2) @binding(5) var material_darkcolorscheme_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = floor(in.uv * pm_common.pixels) / pm_common.pixels;

    var d_light = distance(uv, pm_common.light_origin);

    uv = rotate(uv, pm_common.rotation);

    var uv_center = uv - vec2<f32>(0.0, 0.5);
    uv_center *= vec2(1.0, pm_ring.ring_perspective);
    
    let center_d = distance(uv_center,vec2<f32>(0.5, 0.0));

    var ring = smoothstep(0.5-pm_ring.ring_width*2.0, 0.5-pm_ring.ring_width, center_d);
	ring *= smoothstep(center_d-pm_ring.ring_width, center_d, 0.4);

    if uv.y < 0.5 {
		ring *= step(1.0/pm_ring.scale_rel_to_planet, distance(uv,vec2<f32>(0.5)));
	}

    uv_center = rotate(uv_center+vec2<f32>(0.0, 0.5), globals.time*pm_common.time_speed);
    ring *= fbm(uv_center*pm_common.size);

    let posterized = floor((ring+pow(d_light, 2.0)*2.0)*4.0)/4.0;

    var col: vec4<f32>;
    if posterized <= 1.0 {
        col = textureSample(material_colorscheme_texture, material_colorscheme_sampler, vec2<f32>(posterized, uv.y));
    } else {
        col = textureSample(material_darkcolorscheme_texture, material_darkcolorscheme_sampler, vec2<f32>(posterized-1.0, uv.y));
    }

    let ring_a = step(0.28, ring);

    return vec4<f32>(col.rgb, ring_a * col.a);
}
