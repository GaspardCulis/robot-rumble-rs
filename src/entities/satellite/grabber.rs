use bevy::math::ops::atan2;
use bevy::prelude::*;
use bevy::text::{JustifyText, Text2d, TextColor, TextFont, TextLayout};
use bevy_ggrs::{GgrsSchedule, LocalPlayers};
use leafwing_input_manager::prelude::ActionState;

use super::{Satellite, SatelliteSet};
use super::{SatelliteConfig, SatelliteConfigHandle};
use crate::core::physics::{Position, Velocity};
use crate::entities::player::{Player, PlayerAction};

const ROPE_MIN_LENGTH: f32 = 50.0;
const ROPE_MAX_LENGTH: f32 = 275.0;
const ROPE_ADJUST_SPEED: f32 = 50.0;
const GRABBER_ENTRY_MARGIN: f32 = 10.0;

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
pub struct GrabbedBy(pub Entity);

#[derive(Component, Clone)]
pub struct GrabbedOrbit {
    pub center: Vec2,
    pub distance: f32,
    pub angle: f32,
    pub initial_speed: f32,
}

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

pub struct GrabberPlugin;
impl Plugin for GrabberPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GgrsSchedule,
            (
                detect_player_entry,
                handle_grabber_interaction,
                adjust_rope_length,
                update_grabbed_players,
                cleanup_grabbed_orbits,
                cleanup_grabber_ropes,
                update_grabber_ropes,
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
    grabber_query: Query<(Entity, &Transform), With<Grabber>>,
    config_handle: Res<SatelliteConfigHandle>,
    configs: Res<Assets<SatelliteConfig>>,
) {
    let Some(config) = configs.get(&config_handle.0) else {
        warn!("Satellite config not loaded yet");
        return;
    };

    for (player_entity, player_position) in player_query.iter() {
        let closest_grabber = grabber_query
            .iter()
            .filter_map(|(entity, transform)| {
                let distance = player_position.distance(transform.translation.truncate()) + 30.0;
                (distance < config.grabber_radius + GRABBER_ENTRY_MARGIN)
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
            .is_some_and(|local| local.0.contains(&player.handle))
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
            &ActionState<PlayerAction>,
            &Position,
            Option<&NearbyGrabber>,
            Option<&GrabbedBy>,
            &Velocity,
        ),
        With<Player>,
    >,
    grabber_query: Query<&Transform, With<Grabber>>,
) {
    for (player_entity, actions, position, nearby, grabbed_by, vel) in player_query.iter_mut() {
        let is_pressed = actions.pressed(&PlayerAction::Interact);

        if is_pressed && grabbed_by.is_none() {
            if let Some(nearby) = nearby
                && let Ok(grabber_tf) = grabber_query.get(nearby.0)
            {
                let center = grabber_tf.translation.truncate();
                let pos = position.0;
                let offset = pos - center;
                let distance = offset.length();
                let angle = atan2(offset.y, offset.x);

                let tangent = if offset.length_squared() > f32::EPSILON {
                    Vec2::new(-offset.y, offset.x).normalize()
                } else {
                    Vec2::ZERO
                };

                let sign = if vel.0.dot(tangent) >= 0.0 { 1.0 } else { -1.0 };
                let mut speed = vel.0.dot(tangent);
                if speed.abs() < 600.0 {
                    speed = 600.0 * sign;
                }

                commands
                    .entity(player_entity)
                    .insert(GrabbedBy(nearby.0))
                    .insert(GrabbedOrbit {
                        center,
                        distance,
                        angle,
                        initial_speed: speed,
                    });
                let mesh = meshes.add(Rectangle::new(4.0, 1.0));
                let material = materials.add(Color::srgb(0.0, 0.0, 1.0));

                commands.spawn((
                    Mesh2d(mesh),
                    MeshMaterial2d(material),
                    Transform::from_translation(((center + pos) / 2.0).extend(1.0))
                        .looking_at((pos - center).extend(0.0), Vec3::Y),
                    GlobalTransform::default(),
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
                .remove::<GrabbedOrbit>();
        }
    }
}

fn update_grabbed_players(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Position,
        &mut Velocity,
        &mut GrabbedOrbit,
        &GrabbedBy,
    )>,
    satellite_query: Query<&Transform, With<Satellite>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (entity, mut pos, mut vel, mut orbit, grabbed_by) in query.iter_mut() {
        if let Ok(sat_tf) = satellite_query.get(grabbed_by.0) {
            orbit.center = sat_tf.translation.truncate();
            orbit.angle += (orbit.initial_speed / orbit.distance) * delta;

            let offset = Vec2::from_angle(orbit.angle) * orbit.distance;
            let tangent = Vec2::new(-offset.y, offset.x).normalize();

            vel.0 = tangent * orbit.initial_speed;
            pos.0 = orbit.center + offset;
        } else {
            commands
                .entity(entity)
                .remove::<GrabbedOrbit>()
                .remove::<GrabbedBy>();
        }
    }
}

#[allow(clippy::disallowed_methods)] // Visual doesn't need determinism
fn update_grabber_ropes(
    mut commands: Commands,
    rope_query: Query<(Entity, &GrabberRope)>,
    player_query: Query<&Transform, With<Player>>,
    satellite_query: Query<&Transform, With<Satellite>>,
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

fn adjust_rope_length(
    mut query: Query<
        (&ActionState<PlayerAction>, &mut GrabbedOrbit),
        (With<Player>, With<GrabbedBy>),
    >,
) {
    for (actions, mut orbit) in query.iter_mut() {
        let delta = match (
            actions.pressed(&PlayerAction::RopeRetract),
            actions.pressed(&PlayerAction::RopeExtend),
        ) {
            (true, false) => -ROPE_ADJUST_SPEED,
            (false, true) => ROPE_ADJUST_SPEED,
            _ => 0.0,
        };

        if delta != 0.0 {
            let old = orbit.distance;
            let new = (old + delta).clamp(ROPE_MIN_LENGTH, ROPE_MAX_LENGTH);

            if old > 1.0 && new != old {
                let speed = orbit.initial_speed * old / new;
                orbit.initial_speed = orbit.initial_speed * 0.5 + speed * 0.5;
            }

            orbit.distance = new;
        }
    }
}

fn cleanup_grabbed_orbits(
    mut commands: Commands,
    query: Query<Entity, (With<GrabbedOrbit>, Without<GrabbedBy>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<GrabbedOrbit>();
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
