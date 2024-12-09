use bevy::prelude::{App, Plugin};

pub mod bullet;
pub mod planet;
pub mod player;

pub struct EntitiesPlugin;
impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bullet::BulletPlugin)
            .add_plugins(planet::PlanetPlugin)
            .add_plugins(player::PlayerPlugin);
    }
}
