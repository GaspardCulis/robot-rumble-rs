use bevy::prelude::*;

use super::physics::{self, Position, Velocity};

const G: f32 = 800.;

#[derive(Component, Debug, Reflect, Clone, PartialEq)]
pub struct Mass(pub u32);

#[derive(Component, Debug, Reflect, Clone, PartialEq)]
pub struct Passive;

#[derive(Component, Debug, Reflect, Clone, PartialEq)]
pub struct Static;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Mass>().add_systems(
            FixedUpdate,
            apply_forces
                .in_set(physics::PhysicsSet)
                .before(physics::update_position),
        );
    }
}

fn apply_forces(
    mut on: Query<(&Mass, &Position, &mut Velocity), Without<Static>>,
    from: Query<(&Mass, &Position), Without<Passive>>,
    time: Res<Time>,
) {
    for (a_mass, a_position, mut a_velocity) in on.iter_mut() {
        let mut force_vec = Vec2::new(0., 0.);
        for (b_mass, b_position) in from.iter() {
            if a_position.0 == b_position.0 {
                continue;
            }

            let distance = a_position.0.distance_squared(b_position.0).ceil();
            let force = G * (a_mass.0 * b_mass.0) as f32 / distance;
            force_vec += force * (b_position.0 - a_position.0).normalize();
        }

        a_velocity.0 += (force_vec / a_mass.0 as f32) * time.delta_secs();
    }
}
