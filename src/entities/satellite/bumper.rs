use crate::core::physics::{Position, Velocity};
use crate::entities::player::Player;
use crate::entities::satellite::Satellite;
use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use super::{SatelliteConfig, SatelliteConfigHandle, SatelliteSet};

use bevy::asset::Assets;

#[derive(Component)]
pub struct Bumper;

pub struct BumperPlugin;

impl Plugin for BumperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GgrsSchedule,
            bumper_push_player.in_set(SatelliteSet::Bumper),
        );
    }
}

fn bumper_push_player(
    bumper_query: Query<&Transform, (With<Satellite>, With<Bumper>)>,
    mut player_query: Query<(&Position, &mut Velocity), With<Player>>,
    config_handle: Res<SatelliteConfigHandle>,
    configs: Res<Assets<SatelliteConfig>>,
) {
    let Some(config) = configs.get(&config_handle.0) else {
        warn!("Satellite config not loaded yet");
        return;
    };

    let bumper_radius = config.bump_radius;
    let bump_multiplier = config.bump_multiplier;

    for bumper_transform in bumper_query.iter() {
        let bumper_pos = bumper_transform.translation.truncate();

        for (player_position, mut velocity) in player_query.iter_mut() {
            let player_pos = player_position.0;

            let distance = player_pos.distance(bumper_pos);

            if distance < bumper_radius {
                let push_dir = (player_pos - bumper_pos).normalize_or_zero();
                let incoming_speed = velocity.0.length();

                let bump_speed = incoming_speed * bump_multiplier;

                velocity.0 = push_dir * bump_speed;
            }
        }
    }
}
