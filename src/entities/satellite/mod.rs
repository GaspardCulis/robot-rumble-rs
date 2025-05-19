use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ggrs::GgrsSchedule;
use crate::core::{physics::PhysicsSet};

mod satellite;
pub mod graviton;
pub mod bumper;
pub mod orbit_material;

pub use satellite::{Satellite, SpawnSatelliteEvent, SatelliteKind};

pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<graviton::Orbited>()
            .add_plugins(RonAssetPlugin::<satellite::SatelliteConfig>::new(&[]))
            .add_plugins(satellite::SatellitePlugin)
            .add_systems(
                GgrsSchedule,
                graviton::detect_player_orbit_entry
                    .in_set(PhysicsSet::Gravity)
                    .after(graviton::update_orbiting_players)
                    .after(graviton::update_orbit_cooldowns)
                    .after(crate::core::physics::update_spatial_bundles),
            )
            .add_systems(Startup, satellite::load_satellite_config);

        graviton::register_graviton_systems(app);
        bumper::register_bumper_systems(app);
    }
}
