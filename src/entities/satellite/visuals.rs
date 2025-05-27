use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct OrbitMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(0)]
    pub saturation: f32,
    #[uniform(0)]
    pub alpha: f32,
    #[uniform(0)]
    pub _wasm_padding: Vec2,
}

impl Material2d for OrbitMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/orbit.wgsl".into()
    }
}

#[allow(clippy::disallowed_methods)] // Visual doesn't need determinism
pub fn generate_ring(inner_radius: f32, outer_radius: f32, resolution: usize) -> Mesh {
    let mut positions = Vec::with_capacity(resolution * 2);
    let mut uvs: Vec<Vec2> = Vec::with_capacity(resolution * 2);
    let mut indices = Vec::with_capacity(resolution * 6);

    for i in 0..resolution {
        let angle = i as f32 / resolution as f32 * std::f32::consts::TAU;
        let dir = Vec2::new(angle.cos(), angle.sin());
        positions.push((dir * outer_radius).extend(0.0));
        positions.push((dir * inner_radius).extend(0.0));

        uvs.push((dir * 0.5 + Vec2::splat(0.5)).into());
        uvs.push((dir * 0.5 + Vec2::splat(0.5)).into());
    }

    for i in 0..resolution {
        let i0 = (i * 2) as u32;
        let i1 = (i * 2 + 1) as u32;
        let i2 = ((i * 2 + 2) % (resolution * 2)) as u32;
        let i3 = ((i * 2 + 3) % (resolution * 2)) as u32;

        indices.extend_from_slice(&[i0, i2, i1, i2, i3, i1]);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0, 0.0, 1.0]; resolution * 2],
    );
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
