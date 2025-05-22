use bevy::prelude::{App, Plugin};

pub mod planet;
pub mod player;
pub mod projectile;
pub mod satellite;
pub struct EntitiesPlugins;
impl Plugin for EntitiesPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(planet::PlanetPlugin)
            .add_plugins(projectile::ProjectilePlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(satellite::SatellitePlugin);
    }
}
