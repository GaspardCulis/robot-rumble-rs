use bevy::math::ops::atan2;
use bevy::prelude::*;

use crate::core::physics::{Position, Velocity};
use crate::entities::player::Player;

use bevy_ggrs::GgrsSchedule;

use super::{SatelliteConfig, SatelliteConfigHandle, SatelliteSet};

#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct Orbited {
    pub center: Vec2,
    pub angular_speed: f32,
    pub time_left: f32,
    pub initial_speed: f32,
    pub angle: f32,
}

#[derive(Component)]
pub struct OrbitCooldown {
    pub timer: Timer,
}

#[derive(Component)]
#[require(Name::new("Graviton"))]
pub struct Graviton;

#[derive(Component)]
pub struct GravitonVisual {
    pub active: Handle<Image>,
    pub inactive: Handle<Image>,
}

pub struct GravitonPlugin;
impl Plugin for GravitonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Orbited>().add_systems(
            GgrsSchedule,
            (
                detect_player_orbit_entry,
                update_orbiting_players,
                update_orbit_cooldowns,
            )
                .chain()
                .in_set(SatelliteSet::Graviton),
        );
    }
}

fn detect_player_orbit_entry(
    mut commands: Commands,
    graviton_query: Query<(&Position, Option<&OrbitCooldown>), With<Graviton>>,
    mut player_query: Query<(Entity, &Position, &Velocity), (With<Player>, Without<Orbited>)>,
    config_handle: Res<SatelliteConfigHandle>,
    configs: Res<Assets<SatelliteConfig>>,
) {
    let Some(config) = configs.get(&config_handle.0) else {
        warn!("Satellite config not loaded yet");
        return;
    };

    let orbit_radius = config.orbit_radius;
    let min_angular_speed = config.min_angular_speed;
    let orbit_duration = config.orbit_duration;

    for (player_entity, player_position, velocity) in player_query.iter_mut() {
        for (graviton_pos, maybe_cooldown) in graviton_query.iter() {
            if let Some(cooldown) = maybe_cooldown
                && !cooldown.timer.finished()
            {
                continue; // graviton encore en cooldown
            }

            let distance = player_position.0.distance(graviton_pos.0);

            let initial_speed = velocity.length();

            let dir = player_position.0 - graviton_pos.0;
            let angle = atan2(dir.y, dir.x);

            if distance < orbit_radius {
                commands.entity(player_entity).insert(Orbited {
                    center: graviton_pos.0,
                    angular_speed: min_angular_speed,
                    time_left: orbit_duration,
                    initial_speed,
                    angle,
                });
                break;
            }
        }
    }
}

fn update_orbiting_players(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &mut Position, &mut Velocity, &mut Orbited),
        Without<Graviton>,
    >,
    graviton_query: Query<(Entity, &Position), With<Graviton>>,
    config_handle: Res<SatelliteConfigHandle>,
    configs: Res<Assets<SatelliteConfig>>,
    time: Res<Time>,
) {
    let Some(config) = configs.get(&config_handle.0) else {
        warn!("Satellite config not loaded yet");
        return;
    };

    let orbit_radius = config.orbit_radius;
    let orbit_duration = config.orbit_duration;
    let orbit_cooldown_duration = config.orbit_cooldown;
    let decay_rate = config.decay_rate;

    for (entity, mut position, mut velocity, mut orbited) in player_query.iter_mut() {
        let delta = time.delta_secs();

        orbited.time_left = (orbited.time_left - delta).max(0.0);

        if orbited.time_left <= 0.0 {
            commands.entity(entity).remove::<Orbited>();

            // On cherche le satellite le plus proche
            // TODO: Use collision plugin
            if let Some((satellite_entity, _)) = graviton_query
                .iter()
                .map(|(entity, position)| (entity, position.0))
                .min_by(|(_, a), (_, b)| {
                    a.distance_squared(position.0)
                        .partial_cmp(&b.distance_squared(position.0))
                        .unwrap()
                })
            {
                // On applique un cooldown au satellite
                commands.entity(satellite_entity).insert(OrbitCooldown {
                    timer: Timer::from_seconds(orbit_cooldown_duration, TimerMode::Once),
                });
            }

            continue;
        }

        // === Calcul de la vitesse courante attendue ===
        let target_speed = orbit_radius * orbited.angular_speed * 2.2;
        let mut current_speed =
            orbited.initial_speed - decay_rate * (orbit_duration - orbited.time_left);
        current_speed = current_speed.max(target_speed);

        // === Déduire la vitesse angulaire à partir de la vitesse orbitale ===
        let angular_speed = current_speed / orbit_radius;

        // === Incrément de l'angle et position ===
        orbited.angle += angular_speed * delta;
        let dir = Vec2::from_angle(orbited.angle) * orbit_radius;
        let tangent = Vec2::new(-dir.y, dir.x).normalize();

        velocity.0 = tangent * current_speed;
        position.0 = orbited.center + dir;
    }
}

fn update_orbit_cooldowns(
    mut cooldown_query: Query<(Entity, &mut OrbitCooldown, &Children), With<Graviton>>,
    mut sprite_query: Query<&mut Sprite>,
    visual_query: Query<&GravitonVisual>,
    time: Res<Time>,
) {
    for (entity, mut cooldown, children) in cooldown_query.iter_mut() {
        cooldown.timer.tick(time.delta());

        if let Ok(visual) = visual_query.get(entity) {
            for child in children.iter() {
                if let Ok(mut sprite) = sprite_query.get_mut(child) {
                    sprite.image = if cooldown.timer.finished() {
                        visual.active.clone()
                    } else {
                        visual.inactive.clone()
                    };
                }
            }
        }
    }
}
