use bevy::{prelude::*, render::primitives::Aabb};
use robot_rumble::{core::collision::CollisionShape, entities::planet};

pub fn update_planet_radius(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<
        (Entity, &planet::Radius, &Mesh2d, &Children),
        (With<planet::Planet>, Changed<planet::Radius>),
    >,
) {
    for (entity, radius, mesh, children) in query.iter() {
        // Update mesh
        let mesh = meshes.get_mut(mesh).unwrap();
        *mesh = Mesh::from(Circle::new(radius.0 as f32));

        // Update material layers
        for material_layer in children {
            commands.entity(*material_layer).despawn();
        }

        // Update collision shape
        commands
            .entity(entity)
            .insert(CollisionShape::Circle(radius.0 as f32));

        // Update mesh Aabb (see https://github.com/bevyengine/bevy/issues/18221)
        commands.entity(entity).remove::<Aabb>();
    }
}

pub fn mouse_pos_to_world(
    mouse_pos: &Vec2,
    camera_transform: &Transform,
    window_size: &Vec2,
) -> Vec2 {
    let abs_mouse_pos = mouse_pos - window_size / 2.0;

    camera_transform
        .transform_point(Vec3::new(abs_mouse_pos.x, -abs_mouse_pos.y, 0.0))
        .xy()
}
