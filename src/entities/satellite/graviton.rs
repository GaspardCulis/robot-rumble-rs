use bevy::prelude::*;

use crate::entities::player::Player; 
use crate::entities::satellite::{Satellite};
use crate::core::physics::{Position, Velocity,PhysicsSet, update_spatial_bundles};

use bevy_ggrs::GgrsSchedule;
use crate::core::gravity::apply_forces;


const ORBIT_RADIUS: f32 = 150.0;
const MIN_ANGULAR_SPEED: f32 = 2.0; // en rad/s, 
const ORBIT_DURATION: f32 = 5.0;    // en secondes
const ORBIT_COOLDOWN_DURATION: f32 = 5.0;


#[derive(Component, Debug, Reflect)]
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
pub struct GravitonVisual {
    pub active: Handle<Image>,
    pub inactive: Handle<Image>,
}


pub fn detect_player_orbit_entry(
    mut commands: Commands,
    graviton_query: Query<(&Transform, Option<&OrbitCooldown>), With<Satellite>>,
    mut player_query: Query<(Entity, &Transform, &Velocity), (With<Player>, Without<Orbited>)>,
) {
    for (player_entity, player_transform, velocity) in player_query.iter_mut() {
        for (graviton_transform, maybe_cooldown) in graviton_query.iter() {
            if let Some(cooldown) = maybe_cooldown {
                if !cooldown.timer.finished() {
                    continue; // graviton encore en cooldown
                }
            }

            let distance = player_transform
                .translation
                .truncate()
                .distance(graviton_transform.translation.truncate());

            let initial_speed = velocity.length();

            let dir = player_transform.translation.truncate() - graviton_transform.translation.truncate();
            let angle = dir.angle_to(Vec2::X);


            if distance < ORBIT_RADIUS {
                commands.entity(player_entity).insert(Orbited {
                    center: graviton_transform.translation.truncate(),
                    angular_speed: MIN_ANGULAR_SPEED,
                    time_left: ORBIT_DURATION,
                    initial_speed,
                    angle,
                });
                break;
            }
        }
    }
}

pub fn update_orbiting_players(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Position, &mut Velocity, &mut Orbited)>,
    graviton_query: Query<(Entity, &Transform), With<Satellite>>,
    time: Res<Time>,
) {
    for (entity, mut position, mut velocity, mut orbited) in query.iter_mut() {
        let delta = time.delta_secs();

        orbited.time_left = (orbited.time_left - delta).max(0.0);

        if orbited.time_left <= 0.0 {
            commands.entity(entity).remove::<Orbited>();

            // On cherche le satellite le plus proche
            if let Some((satellite_entity, _)) = graviton_query
                .iter()
                .map(|(e, t)| (e, t.translation.truncate()))
                .min_by(|(_, a), (_, b)| {
                    a.distance_squared(position.0)
                        .partial_cmp(&b.distance_squared(position.0))
                        .unwrap()
                })
            {
                // On applique un cooldown au satellite
                commands.entity(satellite_entity).insert(OrbitCooldown {
                    timer: Timer::from_seconds(ORBIT_COOLDOWN_DURATION, TimerMode::Once),
                });
            }

            continue;
        }

        // === Calcul de la vitesse courante attendue ===
        let target_speed = ORBIT_RADIUS * orbited.angular_speed * 2.2;
        let decay_rate = 40.0;
        let mut current_speed = orbited.initial_speed - decay_rate * (ORBIT_DURATION - orbited.time_left);
        current_speed = current_speed.max(target_speed);

        // === Déduire la vitesse angulaire à partir de la vitesse orbitale ===
        let angular_speed = current_speed / ORBIT_RADIUS;

        // === Incrément de l'angle et position ===
        orbited.angle += angular_speed * delta;
        let dir = Vec2::from_angle(orbited.angle) * ORBIT_RADIUS;
        let tangent = Vec2::new(-dir.y, dir.x).normalize();

        velocity.0 = tangent * current_speed;
        position.0 = orbited.center + dir;
    }
}


pub fn update_orbit_cooldowns(
    mut cooldown_query: Query<(Entity, &mut OrbitCooldown, &Children)>,
    mut sprite_query: Query<&mut Sprite>,
    visual_query: Query<&GravitonVisual>,
    time: Res<Time>,
) {
    for (entity, mut cooldown, children) in cooldown_query.iter_mut() {
        cooldown.timer.tick(time.delta());

        if let Ok(visual) = visual_query.get(entity) {
            for &child in children.iter() {
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




pub fn register_graviton_systems(app: &mut App) {
    app.register_type::<Orbited>()
        .add_systems(
            GgrsSchedule,
            update_orbiting_players
                .in_set(PhysicsSet::Gravity)
                .after(apply_forces), 
        )
        .add_systems(
            GgrsSchedule,
            update_spatial_bundles
                .in_set(PhysicsSet::Gravity)
                .after(update_orbiting_players), 
        )
        .add_systems(
            GgrsSchedule,
            update_orbit_cooldowns
                .in_set(PhysicsSet::Gravity)
                .before(detect_player_orbit_entry),
        );
}