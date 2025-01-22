use bevy::prelude::*;
use lightyear::prelude::client::{Interpolated, Predicted};
use robot_rumble_common::{core::physics::Velocity, entities::bullet::Bullet};

pub struct ClientBulletPlugin;
impl Plugin for ClientBulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_sprite, rotate_sprite).chain());
    }
}

fn add_sprite(
    mut commands: Commands,
    query: Query<Entity, (Or<(Added<Predicted>, Added<Interpolated>)>, With<Bullet>)>,
    asset_server: Res<AssetServer>,
) {
    for bullet in query.iter() {
        commands.entity(bullet).insert((
            Name::new("Bullet"),
            SpriteBundle {
                texture: asset_server.load("bullet.png"),
                ..Default::default()
            },
        ));
    }
}

fn rotate_sprite(mut query: Query<(&mut Transform, &Velocity), (With<Bullet>, With<Sprite>)>) {
    for (mut transform, velocity) in query.iter_mut() {
        let angle = velocity.angle_between(Vec2::X);
        transform.rotation = Quat::from_rotation_z(-angle);
    }
}
