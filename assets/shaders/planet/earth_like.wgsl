#import bevy_pbr::{mesh_view_bindings::globals,forward_io::VertexOutput}

struct PlanetMaterial {
    color: vec4<f32>
}

@group(1) @binding(0) var<uniform> planet_material: PlanetMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return planet_material.color;
}
