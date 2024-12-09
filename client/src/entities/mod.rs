use bevy::prelude::{App, Plugin};

pub mod bullet;
pub mod planet;
pub mod player;

pub struct ClientEntitiesPlugins;
impl Plugin for ClientEntitiesPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(bullet::ClientBulletPlugin)
            .add_plugins(planet::ClientPlanetPlugin)
            .add_plugins(player::ClientPlayerPlugin);
    }
}
