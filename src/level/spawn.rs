use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;
use rand::{Rng as _, SeedableRng as _, seq::IteratorRandom};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    core::physics::{PhysicsSet, Position, Rotation, Velocity},
    entities::{
        planet::{Planet, Radius},
        player::{PLAYER_RADIUS, Percentage, Player},
    },
    network::SessionSeed,
};

pub struct MapSpawnPlugin;
impl Plugin for MapSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GgrsSchedule,
            spawn_players
                .before(PhysicsSet::Player)
                .run_if(any_with_component::<Planet>),
        );
    }
}

fn spawn_players(
    mut commands: Commands,
    players_query: Query<(Entity, &Player), Without<Position>>,
    planets_query: Query<(&Position, &Radius), With<Planet>>,
    session_seed: Res<SessionSeed>,
    frame_count: Res<bevy_ggrs::RollbackFrameCount>,
) {
    for (player_entity, player_marker) in players_query.iter() {
        let seed = session_seed
            .0
            .saturating_add((player_marker.handle + frame_count.0 as usize) as u64);
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

        let (spawn_planet_pos, spawn_planet_radius) = planets_query
            .iter()
            .sort::<&Position>()
            .choose(&mut rng)
            .expect("Should not be empty");

        let random_direction = Vec2::from_angle(rng.random::<f32>() * 2. * std::f32::consts::PI);
        let position =
            spawn_planet_pos.0 + random_direction * (spawn_planet_radius.0 as f32 + PLAYER_RADIUS);

        info!("Spawned player {} at {:?}", player_marker.handle, position);

        commands.entity(player_entity).insert((
            Position(position),
            Velocity::default(),
            Rotation::default(), // TODO: Align properly
            Percentage::default(),
        ));
    }
}
