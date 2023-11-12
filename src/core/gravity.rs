use bevy::prelude::*;

const G: u32 = 800;

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

#[derive(Component)]
pub struct Passive;

#[derive(Component)]
pub struct Static;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_forces);
    }
}

pub fn apply_forces(
    mut on: Query<(&Mass, &Position, &mut Velocity), Without<Static>>,
    from: Query<(&Mass, &Position), Without<Passive>>,
    time: Res<Time>,
) {
    for (a_mass, a_position, mut a_velocity) in on.iter_mut() {
        for (b_mass, b_position) in from.iter() {
            todo!();
        }
    }
}
