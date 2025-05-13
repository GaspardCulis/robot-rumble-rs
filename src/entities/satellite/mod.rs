use bevy::prelude::*;

mod satellite;

pub use satellite::{SpawnSatelliteEvent}; 


pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(satellite::SatellitePlugin);
    }
}
