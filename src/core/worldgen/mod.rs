use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use rand::Rng;
use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng as _};
use serde::{Deserialize, Serialize};

#[cfg(feature = "dev_tools")]
use crate::entities::planet::{PlanetType, Radius, SpawnPlanetEvent};
use crate::entities::satellite::{SatelliteKind, SpawnSatelliteEvent};
use crate::utils::geo::{get_aabb, is_circle_inside_convex_polygon, is_point_in_convex_polygon};
use crate::{core::worldgen::voronoi::*, utils::poisson::poisson_sample_in_aabb};

use super::physics::Position;
mod voronoi;

pub struct WorldgenPlugin;
impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<WorldgenConfig>::new(&[]))
            .add_plugins(voronoi::VoronoiPlugin)
            .add_event::<GenerateWorldEvent>()
            .add_systems(Startup, load_worldgen_config)
            .add_systems(
                Update,
                (
                    handle_genworld_event,
                    #[cfg(feature = "dev_tools")]
                    handle_config_reload.run_if(resource_exists::<crate::network::SessionSeed>),
                ),
            );
    }
}

fn load_worldgen_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let config: Handle<WorldgenConfig> = asset_server.load("config/worldgen.ron");
    commands.insert_resource(WorldgenConfigHandle(config));
}

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct WorldgenConfig {
    pub central_star_radius: u32,

    pub min_clusters: u32,
    pub max_clusters: u32,

    pub min_cluster_planet_radius: u32,
    pub max_cluster_planet_radius: u32,

    pub cluster_relaxation: usize,
    pub min_planet_surface_distance: u32,

    pub edge_radius: u32,
    pub edge_margin: u32,

    // Satellite generation
    min_satellites: u32,
    max_satellites: u32,

    satellite_min_distance: f32,
    satellite_max_distance: f32,

    satellite_planet_min_distance: f32,
    satellite_satellite_min_distance: f32,
}

#[derive(Resource)]
pub struct WorldgenConfigHandle(pub Handle<WorldgenConfig>);

#[derive(Component, Debug, Reflect, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerationSeed(pub u64);

#[derive(Event)]
pub struct GenerateWorldEvent {
    pub seed: u64,
}

fn handle_genworld_event(
    mut events: EventReader<GenerateWorldEvent>,
    mut planet_spawn_events: EventWriter<SpawnPlanetEvent>,
    mut satellite_spawn_events: EventWriter<SpawnSatelliteEvent>,
    mut voronoi_drawing_event: EventWriter<VoronoiGeneratedEvent>,
    config_handle: Res<WorldgenConfigHandle>,
    configs: Res<Assets<WorldgenConfig>>,
) {
    let Some(config) = configs.get(&config_handle.0) else {
        warn!("Worldgen config not loaded yet");
        return;
    };

    for GenerateWorldEvent { seed } in events.read() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(*seed);
        let mut planets = Vec::new();
        planets.push(SpawnPlanetEvent {
            position: Position(Vec2::ZERO),
            radius: Radius(config.central_star_radius),
            r#type: PlanetType::Star,
            seed: rng.random(),
        });

        let num_planets = rng.random_range(config.min_clusters..config.max_clusters) as usize;
        // Générer un certain nombre de satellites

        let num_satellites =
            rng.random_range(config.min_satellites..config.max_satellites) as usize;

        let effective_radius = (config.edge_radius - config.edge_margin) as f32;
        // Pick cluster centers using Poisson sampling
        let mut positions: Vec<Vec2> = poisson_sample_in_aabb(
            Vec2::ZERO - effective_radius,
            Vec2::ZERO + effective_radius,
            850.,
            100,
            *seed,
            2 * (num_planets + num_satellites),
        );

        // Add Sun before building
        positions.push(Vec2::ZERO);

        // Build a Voronoi diagram
        let relaxation = rng.random_range(0..config.cluster_relaxation) as usize;
        let diagram = build_voronoi(positions, 2.0 * config.edge_radius as f64, relaxation);
        let (polygons, centroids) = adjust_to_circle(diagram, config.edge_radius as f32);
        voronoi_drawing_event.write(VoronoiGeneratedEvent {
            polygons: polygons.clone(),
            centroids: centroids.clone(),
        });

        // Generate clusters
        for i in 0..num_planets {
            let position = centroids[i];
            let radius = rng
                .random_range(config.min_cluster_planet_radius..config.max_cluster_planet_radius);
            info!("Generating planet at ({},{})", position.x, position.y);
            let distance = position.distance(Vec2::ZERO);
            if distance < (radius + config.central_star_radius) as f32 {
                // This is a part of the Sun's cluster then
                info!("Sun's cluster, skipping");
                continue;
            }
            // add core
            planets.push(SpawnPlanetEvent {
                position: Position(position),
                radius: Radius(radius),
                r#type: PlanetType::Planet,
                seed: rng.random(),
            });
        }

        for j in num_planets..num_satellites + num_planets {
            let position = centroids[j];
            let radius = 300;
            info!("Generating satelite at ({},{})", position.x, position.y);
            let distance = position.distance(Vec2::ZERO);
            if distance < (radius + config.central_star_radius) as f32 {
                // This is a part of the Sun's cluster then
                info!("Sun's cluster, skipping");
                continue;
            }
            let kind = match rng.random_range(0..3) {
                0 => SatelliteKind::Graviton,
                1 => SatelliteKind::Bumper,
                _ => SatelliteKind::Grabber,
            };
            satellite_spawn_events.write(SpawnSatelliteEvent {
                position: Position(position.clone()),
                // FIX: Config file or constant, will be done when we have better sprites
                scale: 0.7,
                kind,
            });
        }

        for spawn_event in planets {
            planet_spawn_events.write(spawn_event);
        }
    }
}

/// Re-generates world on config changes. Will cause desyncs
#[cfg(feature = "dev_tools")]
fn handle_config_reload(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<WorldgenConfig>>,
    mut worldgen_events: EventWriter<GenerateWorldEvent>,
    entities: Query<
        Entity,
        Or<(
            With<crate::entities::planet::Planet>,
            With<crate::entities::satellite::Satellite>,
            With<ClusterCell>,
        )>,
    >,
    seed: Res<crate::network::SessionSeed>,
) {
    for event in events.read() {
        if let AssetEvent::Modified { id: _ } = event {
            for entity in entities.iter() {
                commands.entity(entity).despawn();
            }

            worldgen_events.write(GenerateWorldEvent { seed: seed.0 });
        };
    }
}
