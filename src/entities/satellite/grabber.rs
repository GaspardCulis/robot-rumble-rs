use bevy::prelude::*;
use bevy::text::{JustifyText, Text2d, TextColor, TextFont, TextLayout};
use bevy_ggrs::{GgrsSchedule, LocalPlayers};

use super::SatelliteSet;
use super::assets::{SatelliteAssets, SatelliteConfig};
use crate::core::inputs::{PlayerAction, PlayerActionState};
use crate::core::physics::{Position, Velocity};
use crate::entities::player::Player;

#[derive(Component)]
#[require(Name::new("Grabber"))]
pub struct Grabber;

#[derive(Component)]
pub struct ShowInteractPrompt {
    pub message: String,
}

#[derive(Component, Clone)]
pub struct NearbyGrabber(pub Entity);

#[derive(Component, Clone)]
pub struct GrabbedBy;

#[derive(Component)]
#[require(Name::new("PlayerPrompt"))]
pub struct PlayerPrompt {
    pub player: Entity,
}

#[derive(Component)]
#[require(Name::new("GrabberRope"))]
pub struct GrabberRope {
    pub player: Entity,
    pub grabber: Entity,
}

#[derive(Component)]
pub struct GrabbedConstraint {
    pub anchor: Entity,
    pub distance: f32,
}

#[derive(Component)]
pub struct GrabberCooldown {
    pub timer: Timer,
}

pub struct GrabberPlugin;
impl Plugin for GrabberPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GgrsSchedule,
            (
                detect_player_entry,
                handle_grabber_interaction,
                update_grabbed_players,
                cleanup_grabber_ropes,
                update_grabber_ropes,
                tick_grabber_cooldown,
            )
                .chain()
                .in_set(SatelliteSet::Grabber),
        );
        app.add_systems(Update, (display_interact_prompt, remove_interact_prompt));
    }
}

fn detect_player_entry(
    mut commands: Commands,
    player_query: Query<(Entity, &Position), With<Player>>,
    grabber_query: Query<(Entity, &Position), With<Grabber>>,
    configs: Res<Assets<SatelliteConfig>>,
    assets: Res<SatelliteAssets>,
) {
    let Some(config) = configs.get(&assets.config) else {
        warn!("Satellite config not loaded yet");
        return;
    };

    for (player_entity, player_position) in player_query.iter() {
        let closest_grabber = grabber_query
            .iter()
            .filter_map(|(entity, position)| {
                let distance = player_position.distance(position.0) + 30.0;
                (distance < config.grabber_radius + config.grabber_entry_margin)
                    .then_some((entity, distance))
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        match closest_grabber {
            Some((grabber_entity, _)) => {
                commands
                    .entity(player_entity)
                    .insert(ShowInteractPrompt {
                        message: "Press E to hang".to_string(),
                    })
                    .insert(NearbyGrabber(grabber_entity));
            }
            None => {
                commands
                    .entity(player_entity)
                    .remove::<ShowInteractPrompt>()
                    .remove::<NearbyGrabber>();
            }
        }
    }
}

fn display_interact_prompt(
    mut commands: Commands,
    player_query: Query<(Entity, &Player, &ShowInteractPrompt, &NearbyGrabber)>,
    grabber_query: Query<&Transform, With<Grabber>>,
    prompt_query: Query<&PlayerPrompt>,
    asset_server: Res<AssetServer>,
    local_players: Option<Res<LocalPlayers>>,
) {
    for (player_entity, player, prompt, nearby_grabber) in player_query.iter() {
        if local_players
            .as_ref()
            .is_some_and(|local| !local.0.contains(&player.handle))
        {
            continue;
        }

        if prompt_query.iter().all(|p| p.player != player_entity)
            && let Ok(grabber_transform) = grabber_query.get(nearby_grabber.0)
        {
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            commands.spawn((
                Text2d::new(prompt.message.clone()),
                TextFont {
                    font,
                    font_size: 30.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                TextLayout {
                    justify: JustifyText::Center,
                    ..Default::default()
                },
                Transform::from_translation(
                    grabber_transform.translation + Vec3::new(0.0, -100.0, 2.0),
                ),
                GlobalTransform::default(),
                PlayerPrompt {
                    player: player_entity,
                },
            ));
        }
    }
}

fn remove_interact_prompt(
    mut commands: Commands,
    prompt_query: Query<(Entity, &PlayerPrompt)>,
    player_query: Query<&ShowInteractPrompt, With<Player>>,
) {
    for (prompt_entity, prompt) in prompt_query.iter() {
        if player_query.get(prompt.player).is_err() {
            commands.entity(prompt_entity).despawn();
        }
    }
}

fn handle_grabber_interaction(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<
        (
            Entity,
            &PlayerActionState,
            &Position,
            Option<&NearbyGrabber>,
            Option<&GrabbedBy>,
            Option<&GrabberCooldown>,
            &mut Velocity,
        ),
        With<Player>,
    >,
    grabber_query: Query<&Position, With<Grabber>>,
) {
    for (player_entity, actions, position, nearby, grabbed_by, grabber_cooldown, mut velocity) in
        player_query.iter_mut()
    {
        if grabber_cooldown.is_some() {
            continue;
        }

        let is_pressed = actions.pressed(&PlayerAction::Interact);

        if is_pressed && grabbed_by.is_none() {
            if let Some(nearby) = nearby
                && let Ok(grabber_pos) = grabber_query.get(nearby.0)
            {
                let center = grabber_pos.0;
                let pos = position.0;
                let offset = pos - center;
                let distance = offset.length();

                // Appliquer une vélocité tangentielle pour commencer l'orbite
                let tangent = Vec2::new(-offset.y, offset.x).normalize_or_zero();
                let tangential_speed = 800.0; // Ajustable
                let direction_sign = if velocity.0.dot(tangent) >= 0.0 {
                    1.0
                } else {
                    -1.0
                };
                velocity.0 = tangent * tangential_speed * direction_sign;

                commands
                    .entity(player_entity)
                    .insert(GrabbedBy)
                    .insert(GrabbedConstraint {
                        anchor: nearby.0,
                        distance,
                    });

                let mesh = meshes.add(Rectangle::new(4.0, 1.0));
                let material = materials.add(Color::srgb(0.0, 0.0, 1.0));
                commands.spawn((
                    Mesh2d(mesh),
                    MeshMaterial2d(material),
                    Transform::from_translation(((center + pos) / 2.0).extend(1.0))
                        .looking_at((pos - center).extend(0.0), Vec3::Y),
                    GrabberRope {
                        player: player_entity,
                        grabber: nearby.0,
                    },
                ));
            }
        } else if !is_pressed && grabbed_by.is_some() {
            commands
                .entity(player_entity)
                .remove::<GrabbedBy>()
                .remove::<GrabbedConstraint>();
        }
    }
}

fn update_grabbed_players(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Position, &mut Velocity, &GrabbedConstraint), With<Player>>,
    anchor_query: Query<&Position, (With<Grabber>, Without<Player>)>,
    assets: Res<SatelliteAssets>,
    configs: Res<Assets<SatelliteConfig>>,
) {
    for (entity, position, mut velocity, constraint) in query.iter_mut() {
        let Some(config) = configs.get(&assets.config) else {
            warn!("Satellite config not loaded yet");
            return;
        };
        if velocity.0.length() > config.max_grabber_speed {
            commands
                .entity(entity)
                .remove::<GrabbedBy>()
                .remove::<GrabbedConstraint>()
                .insert(GrabberCooldown {
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                });
            continue;
        }
        if let Ok(anchor_pos) = anchor_query.get(constraint.anchor) {
            let offset = position.0 - anchor_pos.0;
            let current_distance = offset.length();

            if current_distance < f32::EPSILON {
                continue;
            }

            let direction = offset / current_distance;
            let target_position = anchor_pos.0 + direction * constraint.distance;

            let stiffness = 5.0;
            let correction = (target_position - position.0) * stiffness;
            velocity.0 += correction;

            let radial_speed = velocity.0.dot(direction);
            velocity.0 -= direction * radial_speed * 0.2;

            let tangent = Vec2::new(-direction.y, direction.x);
            let tangential_speed = velocity.0.dot(tangent);
            let orbit_boost = 0.05;
            velocity.0 += tangent * tangential_speed.signum() * orbit_boost;
        }
    }
}

#[allow(clippy::disallowed_methods)] // Visual doesn't need determinism
fn update_grabber_ropes(
    mut commands: Commands,
    rope_query: Query<(Entity, &GrabberRope)>,
    player_query: Query<&Transform, With<Player>>,
    satellite_query: Query<&Transform, With<Grabber>>,
) {
    for (entity, rope) in rope_query.iter() {
        match (
            player_query.get(rope.player),
            satellite_query.get(rope.grabber),
        ) {
            (Ok(p_tf), Ok(g_tf)) => {
                let player = p_tf.translation.truncate();
                let grabber = g_tf.translation.truncate();
                let mid = (player + grabber) / 2.0;
                let dir = player - grabber;
                let rot = Quat::from_rotation_z(dir.y.atan2(dir.x) + std::f32::consts::FRAC_PI_2);

                commands.entity(entity).insert(Transform {
                    translation: mid.extend(3.0),
                    rotation: rot,
                    scale: Vec3::new(1.0, dir.length(), 1.0),
                });
            }
            _ => commands.entity(entity).despawn(),
        }
    }
}

fn cleanup_grabber_ropes(
    mut commands: Commands,
    rope_query: Query<(Entity, &GrabberRope)>,
    player_query: Query<Option<&GrabbedBy>, With<Player>>,
) {
    for (entity, rope) in rope_query.iter() {
        if let Ok(None) = player_query.get(rope.player) {
            commands.entity(entity).despawn();
        }
    }
}

fn tick_grabber_cooldown(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut GrabberCooldown)>,
) {
    for (entity, mut cooldown) in query.iter_mut() {
        cooldown.timer.tick(time.delta());
        if cooldown.timer.finished() {
            commands.entity(entity).remove::<GrabberCooldown>();
        }
    }
}
