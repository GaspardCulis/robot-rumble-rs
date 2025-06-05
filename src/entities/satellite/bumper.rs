use crate::core::physics::{Position, Velocity};
use crate::entities::player::Player;
use crate::entities::satellite::Satellite;
use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use super::SatelliteSet;
use super::assets::{SatelliteAssets, SatelliteConfig};

use bevy::asset::Assets;

#[derive(Component)]
#[require(Name::new("Bumper"))]
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
    bumper_query: Query<&Position, (With<Satellite>, With<Bumper>)>,
    mut player_query: Query<(&Position, &mut Velocity), With<Player>>,
    configs: Res<Assets<SatelliteConfig>>,
    assets: Res<SatelliteAssets>,
) {
    let Some(config) = configs.get(&assets.config) else {
        warn!("Satellite config not loaded yet");
        return;
    };

    let bumper_radius = config.bump_radius;
    let bump_multiplier = config.bump_multiplier;

    for bumper_pos in bumper_query.iter() {
        for (player_pos, mut velocity) in player_query.iter_mut() {
            let distance = player_pos.distance(bumper_pos.0);

            if distance < bumper_radius {
                let push_dir = (player_pos.0 - bumper_pos.0).normalize_or_zero();
                let incoming_speed = velocity.0.length();

                let bump_speed = incoming_speed * bump_multiplier;

                velocity.0 = push_dir * bump_speed;
            }
        }
    }
}
