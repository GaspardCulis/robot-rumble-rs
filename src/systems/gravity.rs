use bevy::prelude::*;

use crate::components::physics::{Mass, Velocity};

pub fn apply_forces(mut query: Query<(&Mass, &mut Velocity)>) {
    for (mass, mut velocity) in query.iter_mut() {
        println!(
            "[TODO] Compute value of velocity {:?} for mass {:?}",
            velocity, mass
        );
    }
}
