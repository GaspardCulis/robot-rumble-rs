use bevy::math::ops::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::core::physics::{Position, Velocity};
use crate::entities::player::{Player, PlayerAction};
use crate::entities::satellite::Satellite;

use bevy_ggrs::GgrsSchedule;

use super::{SatelliteConfig, SatelliteConfigHandle, SatelliteSet};

#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct Orbited {
    pub center: Vec2,
    pub time_left: f32,
    pub initial_speed: f32,
    pub entry_pos: Vec2,
    pub elapsed: f32,
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

#[derive(Component)]
pub struct EjectionArrow {
    pub player: Entity,
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
                update_ejection_arrows,
            )
                .chain()
                .in_set(SatelliteSet::Graviton),
        );
    }
}

fn detect_player_orbit_entry(
    mut commands: Commands,
    graviton_query: Query<(&Transform, Option<&OrbitCooldown>), (With<Satellite>, With<Graviton>)>,
    mut player_query: Query<(Entity, &Position, &Velocity), (With<Player>, Without<Orbited>)>,
    config_handle: Res<SatelliteConfigHandle>,
    configs: Res<Assets<SatelliteConfig>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Some(config) = configs.get(&config_handle.0) else {
        warn!("Satellite config not loaded yet");
        return;
    };

    let orbit_radius = config.orbit_radius;
    let orbit_duration = config.orbit_duration;

    for (player_entity, player_position, velocity) in player_query.iter_mut() {
        for (graviton_transform, maybe_cooldown) in graviton_query.iter() {
            if let Some(cooldown) = maybe_cooldown {
                if !cooldown.timer.finished() {
                    continue; // graviton encore en cooldown
                }
            }

            let graviton_pos = graviton_transform.translation.truncate();
            let distance = player_position.0.distance(graviton_pos);

            let initial_speed = velocity.length();

            let entry_pos = player_position.0;
            let offset = player_position.0 - graviton_pos;
            let angle = atan2(offset.y, offset.x);

            if distance < orbit_radius {
                commands.entity(player_entity).insert(Orbited {
                    center: graviton_pos,
                    time_left: orbit_duration,
                    initial_speed,
                    entry_pos,
                    elapsed: 0.0,
                    angle,
                });
                let center = graviton_pos;
                let direction = (entry_pos - center).normalize();
                let arrow_distance = (entry_pos - center).length() + 30.0;
                let arrow_pos = center + direction * arrow_distance;
                let arrow_angle = atan2(direction.y, direction.x);

                // Créer les deux meshes
                let shaft = meshes.add(Rectangle::new(6.0, 24.0));
                let material = materials.add(Color::srgb(1.0, 0.8, 0.0));

                // Corps de flèche (rotation +90°)
                commands.spawn((
                    Mesh2d(shaft),
                    MeshMaterial2d(material.clone()),
                    Transform {
                        translation: arrow_pos.extend(2.0),
                        rotation: Quat::from_rotation_z(arrow_angle + std::f32::consts::FRAC_PI_2),
                        ..default()
                    },
                    GlobalTransform::default(),
                    EjectionArrow {
                        player: player_entity,
                    },
                ));
                break;
            }
        }
    }
}

fn update_orbiting_players(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Position,
        &mut Velocity,
        &mut Orbited,
        &ActionState<PlayerAction>,
    )>,
    graviton_query: Query<(Entity, &Transform), (With<Satellite>, With<Graviton>)>,
    config_handle: Res<SatelliteConfigHandle>,
    configs: Res<Assets<SatelliteConfig>>,
    time: Res<Time>,
) {
    let Some(config) = configs.get(&config_handle.0) else {
        warn!("Satellite config not loaded yet");
        return;
    };

    for (entity, mut position, mut velocity, mut orbited, input) in query.iter_mut() {
        let delta = time.delta_secs();
        orbited.elapsed += delta;
        orbited.time_left = (orbited.time_left - delta).max(0.0);

        let t = orbited.elapsed / 1.0;

        // Phase 1 : arrivée vers le centre du satellite
        if orbited.elapsed < 1.0 {
            let eased_t = 1.0 - powf(1.0 - t, 2.0);
            position.0 = orbited.entry_pos.lerp(orbited.center, eased_t);

            let direction = (orbited.center - orbited.entry_pos).normalize_or_zero();
            velocity.0 = direction * orbited.initial_speed * (1.0 - t);
            continue;
        }

        // Phase 2 : choisir la direction d'éjection
        let mut direction = 0.0;
        if input.pressed(&PlayerAction::Right) {
            direction -= 1.0;
        }
        if input.pressed(&PlayerAction::Left) {
            direction += 1.0;
        }

        let angular_speed = std::f32::consts::PI;
        orbited.angle += direction * angular_speed * delta;
        orbited.angle = orbited.angle.rem_euclid(2.0 * std::f32::consts::PI);

        // Le joueur reste au centre mais peut choisir son angle
        position.0 = orbited.center;
        velocity.0 = Vec2::ZERO;

        // Ejection une fois que le temps est écoulé
        if orbited.time_left <= 0.0 {
            let eject_dir = Vec2::from_angle(orbited.angle);
            velocity.0 = eject_dir * orbited.initial_speed;
            commands.entity(entity).remove::<Orbited>();

            // Cooldown du graviton
            if let Some((graviton_entity, _)) = graviton_query.iter().min_by(|(_, a), (_, b)| {
                a.translation
                    .truncate()
                    .distance_squared(position.0)
                    .partial_cmp(&b.translation.truncate().distance_squared(position.0))
                    .unwrap()
            }) {
                commands.entity(graviton_entity).insert(OrbitCooldown {
                    timer: Timer::from_seconds(config.orbit_cooldown, TimerMode::Once),
                });
            }
        }
    }
}

fn update_ejection_arrows(
    mut commands: Commands,
    mut arrow_query: Query<(Entity, &mut Transform, &EjectionArrow)>,
    player_query: Query<Option<&Orbited>, With<Player>>,
    config_handle: Res<SatelliteConfigHandle>,
    configs: Res<Assets<SatelliteConfig>>,
) {
    let Some(config) = configs.get(&config_handle.0) else {
        warn!("Satellite config not loaded yet");
        return;
    };
    for (arrow_entity, mut transform, arrow) in arrow_query.iter_mut() {
        if let Ok(Some(orbited)) = player_query.get(arrow.player) {
            let center = orbited.center;
            let direction = Vec2::from_angle(orbited.angle);
            let arrow_distance = config.orbit_radius + 30.0;

            let arrow_pos = center + direction * arrow_distance;
            transform.translation = arrow_pos.extend(transform.translation.z);
            transform.rotation = Quat::from_rotation_z(
                atan2(direction.y, direction.x) + std::f32::consts::FRAC_PI_2,
            );
        } else {
            commands.entity(arrow_entity).despawn();
        }
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
