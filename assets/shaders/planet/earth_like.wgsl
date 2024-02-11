#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
    
struct PlanetMaterial {
    color: vec4<f32>
}

@group(1) @binding(0) var<uniform> planet_material: PlanetMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return planet_material.color * in.uv.x;
}
