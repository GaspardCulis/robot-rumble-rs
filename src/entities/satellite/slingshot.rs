use bevy::math::ops::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::core::physics::{Position, Velocity};
use crate::entities::player::{Player};
use crate::core::inputs::PlayerAction;
use crate::entities::satellite::Satellite;

use bevy_ggrs::GgrsSchedule;

use super::{SatelliteConfig, SatelliteAssets, SatelliteSet};

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
#[require(Name::new("Slingshot"))]
pub struct Slingshot;

#[derive(Component)]
pub struct SlingshotVisual {
    pub active: Handle<Image>,
    pub inactive: Handle<Image>,
}

#[derive(Component)]
pub struct EjectionArrow {
    pub player: Entity,
}

#[derive(Component)]
pub struct SlingshotCord;

#[derive(Resource)]
pub struct SlingshotCordFrames(pub Vec<Handle<Image>>);

#[derive(Component)]
pub struct SlingshotCordAnimation {
    pub timer: Timer,
}

#[derive(Component)]
pub struct SlingshotCordTarget {
    pub target: Entity,
}

#[derive(Component)]
pub struct WasInsideOrbitZone;


pub struct SlingshotPlugin;
impl Plugin for SlingshotPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Orbited>().add_systems(
            GgrsSchedule,
            (
                detect_player_orbit_entry,
                update_orbiting_players,
                update_orbit_cooldowns,
                mark_players_in_orbit_zone,
                cleanup_orbit_zone_flags,
                update_ejection_arrows,
                load_slingcord_frames,
                animate_slingshot_cord,
                update_slingcord_transform,
            )
                .chain()
                .in_set(SatelliteSet::Slingshot),
        );
    }
}

fn detect_player_orbit_entry(
    mut commands: Commands,
    slingshot_query: Query<
        (Entity, &Position, Option<&OrbitCooldown>),
        (With<Satellite>, With<Slingshot>),
    >,
    mut player_query: Query<(Entity, &Position, &Velocity), (With<Player>, Without<Orbited>, Without<WasInsideOrbitZone>)>,
    assets: Res<SatelliteAssets>,
    configs: Res<Assets<SatelliteConfig>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let Some(config) = configs.get(&assets.config) else {
        warn!("Satellite config not loaded yet");
        return;
    };

    let orbit_radius = config.orbit_radius;
    let orbit_duration = config.orbit_duration;

    for (player_entity, player_position, velocity) in player_query.iter_mut() {
        for (slingshot_entity, slingshot_pos, maybe_cooldown) in slingshot_query.iter() {
            if let Some(cooldown) = maybe_cooldown
                && !cooldown.timer.finished()
            {
                continue; // slingshot encore en cooldown
            }

            let distance = player_position.0.distance(slingshot_pos.0);

            let initial_speed = velocity.length();

            let entry_pos = player_position.0;
            let offset = player_position.0 - slingshot_pos.0;
            let angle = atan2(offset.y, offset.x);

            if distance < orbit_radius {
                commands.entity(player_entity).insert(Orbited {
                    center: slingshot_pos.0,
                    time_left: orbit_duration,
                    initial_speed,
                    entry_pos,
                    elapsed: 0.0,
                    angle,
                });
                let center = slingshot_pos.0;
                let direction = (entry_pos - center).normalize();
                let arrow_distance = (entry_pos - center).length() + 30.0;
                let arrow_pos = center + direction * arrow_distance;
                let arrow_angle = atan2(direction.y, direction.x);

                // Créer les deux meshes
                let shaft = meshes.add(Rectangle::new(12.0, 48.0));
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

                commands.entity(slingshot_entity).with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            image: asset_server.load("img/satellites/slingshot/rope_1.png"),
                            ..default()
                        },
                        Transform {
                            translation: Vec3::new(0.0, 0.0, 1.0),
                            ..default()
                        },
                        GlobalTransform::default(),
                        SlingshotCord,
                        SlingshotCordTarget {
                            target: player_entity,
                        },
                        SlingshotCordAnimation {
                            timer: Timer::from_seconds(1.0, TimerMode::Once),
                        },
                    ));
                });
                break;
            }
        }
    }
    for (entity, _, _) in player_query.iter() {
        commands.entity(entity).remove::<WasInsideOrbitZone>();
    }
}

// Retarde le despawn de la corde jusqu'à ce que l'animation soit terminée
fn update_orbiting_players(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Position,
            &mut Velocity,
            &mut Orbited,
            &ActionState<PlayerAction>,
        ),
        Without<Slingshot>,
    >,
    slingshot_query: Query<(Entity, &Position), (With<Satellite>, With<Slingshot>)>,
    cord_query: Query<(Entity, &SlingshotCordTarget, &SlingshotCordAnimation)>,
    assets: Res<SatelliteAssets>,
    configs: Res<Assets<SatelliteConfig>>,
    time: Res<Time>,
) {
    let Some(config) = configs.get(&assets.config) else {
        warn!("Satellite config not loaded yet");
        return;
    };

    for (entity, mut position, mut velocity, mut orbited, input) in query.iter_mut() {
        let delta = time.delta_secs();
        orbited.elapsed += delta;
        orbited.time_left = (orbited.time_left - delta).max(0.0);

        // Phase 1 : arrivée vers le centre du satellite
        if orbited.elapsed < 1.2 {
            let eased_t = 1.0 - powf(1.0 - orbited.elapsed, 2.0);
            position.0 = orbited.entry_pos.lerp(orbited.center, eased_t);

            let direction = (orbited.center - orbited.entry_pos).normalize_or_zero();
            velocity.0 = direction * orbited.initial_speed * (1.0 - orbited.elapsed);
        } else {
            position.0 = orbited.center;
            velocity.0 = Vec2::ZERO;
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

        // Ejection une fois que le temps est écoulé
        if orbited.time_left <= 0.0 {
            let eject_dir = Vec2::from_angle(orbited.angle);
            velocity.0 = eject_dir * orbited.initial_speed;
            commands.entity(entity).remove::<Orbited>();

            let animation_duration = 1.0 / orbited.initial_speed * 2.0; // Ajuste la durée de l'animation
            commands.entity(entity).with_children(|parent| {
                parent.spawn((SlingshotCordAnimation {
                    timer: Timer::from_seconds(animation_duration, TimerMode::Once),
                },));
            });

            // **Ne pas despawn immédiatement la corde ici** !
            // Retarde le despawn à la fin de l'animation
            for (cord_entity, cord_target, animation) in cord_query.iter() {
                if cord_target.target == entity {
                    // Si l'animation est terminée, on despawn la corde
                    if animation.timer.finished() {
                        commands.entity(cord_entity).despawn();
                    }
                }
            }

            // Cooldown du slingshot
            if let Some((slingshot_entity, _)) = slingshot_query.iter().min_by(|(_, a), (_, b)| {
                a.0.distance_squared(position.0)
                    .partial_cmp(&b.0.distance_squared(position.0))
                    .unwrap()
            }) {
                commands.entity(slingshot_entity).insert(OrbitCooldown {
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
    assets: Res<SatelliteAssets>,
    configs: Res<Assets<SatelliteConfig>>,
) {
    let Some(config) = configs.get(&assets.config) else {
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
    mut cooldown_query: Query<(Entity, &mut OrbitCooldown, &Children), With<Slingshot>>,
    mut sprite_query: Query<&mut Sprite>,
    visual_query: Query<&SlingshotVisual>,
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

fn load_slingcord_frames(asset_server: Res<AssetServer>, mut commands: Commands) {
    let frames = vec![
        asset_server.load("img/satellites/slingshot/rope_1.png"),
        asset_server.load("img/satellites/slingshot/rope_2.png"),
        asset_server.load("img/satellites/slingshot/rope_3.png"),
        asset_server.load("img/satellites/slingshot/rope_4.png"),
    ];
    commands.insert_resource(SlingshotCordFrames(frames));
}

fn animate_slingshot_cord(
    time: Res<Time>,
    frames: Res<SlingshotCordFrames>,
    mut query: Query<(&mut Sprite, &mut SlingshotCordAnimation)>,
) {
    for (mut sprite, mut animation) in query.iter_mut() {
        animation.timer.tick(time.delta());

        let progress = animation.timer.elapsed_secs().min(1.0);
        let frame_index = (progress / (1.0 / frames.0.len() as f32)).floor() as usize;

        if let Some(image) = frames.0.get(frame_index.min(frames.0.len() - 1)) {
            sprite.image = image.clone();
        }
    }
}

fn update_slingcord_transform(
    player_query: Query<(&Position, Option<&Orbited>), With<Player>>,
    mut cord_query: Query<(&mut Transform, &SlingshotCordTarget), With<SlingshotCord>>,
    assets: Res<SatelliteAssets>,
    configs: Res<Assets<SatelliteConfig>>,
) {
    let Some(config) = configs.get(&assets.config) else {
        warn!("Satellite config not loaded yet");
        return;
    };

    let orbit_radius = config.orbit_radius;

    for (mut transform, target) in cord_query.iter_mut() {
        if let Ok((_, Some(orbited))) = player_query.get(target.target) {
            let center = orbited.center;
            let angle = orbited.angle;

            // On définit un petit angle de décalage pour simuler les deux attaches
            let delta_angle = 0.5; // ~28.6° (ajuste si nécessaire)

            let left_anchor = center + Vec2::from_angle(angle + delta_angle) * orbit_radius;
            let right_anchor = center + Vec2::from_angle(angle - delta_angle) * orbit_radius;

            // Point milieu entre les deux attaches
            let midpoint = (left_anchor + right_anchor) * 0.5;
            let direction = right_anchor - left_anchor;
            let distance = direction.length();
            let angle_z = atan2(direction.y, direction.x);

            // Position relative à son parent (le satellite)
            transform.translation = (midpoint - center).extend(1.0);
            transform.scale = Vec3::new(distance / 20.0, 8.0, 1.0); // ajuste les diviseurs à ton sprite
            transform.rotation = Quat::from_rotation_z(angle_z);
        }
    }
}

fn mark_players_in_orbit_zone(
    mut commands: Commands,
    slingshot_query: Query<(&Position, Option<&OrbitCooldown>), (With<Slingshot>, With<Satellite>)>,
    player_query: Query<(Entity, &Position), With<Player>>,
    configs: Res<Assets<SatelliteConfig>>,
    assets: Res<SatelliteAssets>,
) {
    let Some(config) = configs.get(&assets.config) else { return; };
    let radius = config.orbit_radius;

    for (player_entity, player_pos) in player_query.iter() {
        for (slingshot_pos, cooldown) in slingshot_query.iter() {
            if let Some(cd) = cooldown {
                if cd.timer.finished() {
                    let dist = player_pos.0.distance(slingshot_pos.0);
                    if dist < radius {
                        commands.entity(player_entity).insert(WasInsideOrbitZone);
                    }
                }
            }
        }
    }
}

fn cleanup_orbit_zone_flags(
    mut commands: Commands,
    player_query: Query<(Entity, &Position), (With<Player>, With<WasInsideOrbitZone>, Without<Orbited>)>,
    slingshot_query: Query<&Position, (With<Slingshot>, With<Satellite>)>,
    configs: Res<Assets<SatelliteConfig>>,
    assets: Res<SatelliteAssets>,
) {
    let Some(config) = configs.get(&assets.config) else { return; };
    let orbit_radius = config.orbit_radius;

    for (player_entity, player_pos) in player_query.iter() {
        let mut still_inside = false;
        for slingshot_pos in slingshot_query.iter() {
            if player_pos.0.distance(slingshot_pos.0) < orbit_radius {
                still_inside = true;
                break;
            }
        }

        if !still_inside {
            commands.entity(player_entity).remove::<WasInsideOrbitZone>();
        }
    }
}
