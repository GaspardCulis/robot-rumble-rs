use bevy::prelude::*;

use crate::core::physics::{PhysicsSet, Position};
use crate::entities::player::Player;
use bevy::text::{Text2d, TextFont, TextColor, TextLayout, JustifyText};
use super::satellite::{SatelliteConfig, SatelliteConfigHandle};
use bevy_ggrs::GgrsSchedule;

#[derive(Component)]
pub struct Grabber;

#[derive(Component)]
pub struct ShowInteractPrompt {
    pub message: String,
}

#[derive(Component)]
pub struct NearbyGrabber(pub Entity);

#[derive(Component)]
pub struct PlayerPrompt {
    pub player: Entity,
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
        let mut closest_grabber: Option<(Entity, f32)> = None;

        for (grabber_entity, grabber_transform) in grabber_query.iter() {
            let distance = player_position.distance(grabber_transform.translation.truncate())+30.0;
            if distance < config.grabber_radius {
                if let Some((_, closest_distance)) = closest_grabber {
                    if distance < closest_distance {
                        closest_grabber = Some((grabber_entity, distance));
                    }
                } else {
                    closest_grabber = Some((grabber_entity, distance));
                }
            }
        }

        if let Some((grabber_entity, _)) = closest_grabber {
            // On est dans le rayon d’un grabber
            commands.entity(player_entity).insert(ShowInteractPrompt {
                message: "E pour s'accrocher".to_string(),
            });
            commands.entity(player_entity).insert(NearbyGrabber(grabber_entity));
        } else {
            // Pas dans le rayon de grabber
            commands.entity(player_entity).remove::<ShowInteractPrompt>();
            commands.entity(player_entity).remove::<NearbyGrabber>();
        }
    }
}


fn display_interact_prompt(
    mut commands: Commands,
    player_query: Query<(Entity, &ShowInteractPrompt, &NearbyGrabber), With<Player>>,
    grabber_query: Query<&Transform, With<Grabber>>,
    prompt_query: Query<&PlayerPrompt>,
    asset_server: Res<AssetServer>,
) {
    for (player_entity, prompt, nearby_grabber) in player_query.iter() {
        if prompt_query.iter().all(|p| p.player != player_entity) {
            if let Ok(grabber_transform) = grabber_query.get(nearby_grabber.0) {
                let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
                commands.spawn((
                    Text2d::new(prompt.message.clone()),
                    TextFont {
                        font: font_handle,
                        font_size: 30.0,
                        ..Default::default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout {
                        justify: JustifyText::Center,
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(
                        grabber_transform.translation.x,
                        grabber_transform.translation.y - 100.0,
                        50.0,
                    )),
                    GlobalTransform::default(),
                    PlayerPrompt { player: player_entity },
                     
                ));
            }
        }
    }
}


fn remove_interact_prompt(
    mut commands: Commands,
    prompt_query: Query<(Entity, &PlayerPrompt)>,
    player_query: Query<&ShowInteractPrompt, With<Player>>,
) {
    for (prompt_entity, player_prompt) in prompt_query.iter() {
        // If the player no longer has ShowInteractPrompt, remove the prompt entity
        if player_query.get(player_prompt.player).is_err() {
            commands.entity(prompt_entity).despawn();
        }
    }
}

pub fn register_grabber_systems(app: &mut App) {
    app.add_systems(
        GgrsSchedule,
        detect_player_entry
            .in_set(PhysicsSet::Gravity)
            .after(crate::core::gravity::apply_forces)
            .before(crate::entities::satellite::graviton::update_orbiting_players),
    );
    app.add_systems(
        GgrsSchedule,
        display_interact_prompt
            .after(detect_player_entry)
            .after(crate::core::gravity::apply_forces)
            .before(crate::entities::satellite::graviton::update_orbiting_players),
    );
    app.add_systems(
        GgrsSchedule,
        remove_interact_prompt
            .after(detect_player_entry)
            .after(crate::core::gravity::apply_forces)
            .before(crate::entities::satellite::graviton::update_orbiting_players),
    );
}