use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
pub struct Mass(pub u32);

pub fn apply_forces(mut query: Query<(&Mass, &mut Velocity)>) {
    for (mass, mut velocity) in query.iter_mut() {
        println!(
            "[TODO] Compute value of velocity {:?} for mass {:?}",
            velocity, mass
        );
    }
}
