use bevy::prelude::*;
use client::is_in_rollback;
use leafwing_input_manager::prelude::ActionState;
use lightyear::prelude::*;

pub const BULLET_SPEED: f32 = 1200.;
pub const BULLET_MASS: u32 = 20;

use crate::{
    core::{
        gravity::{Mass, Passive},
        physics::{Position, Velocity},
    },
    network::protocol::PLAYER_REPLICATION_GROUP,
};

use super::{
    planet::Radius,
    player::{Player, PlayerAction},
};

#[derive(Component)]
pub struct Bullet;

#[derive(Bundle)]
pub struct BulletBundle {
    pub name: Name,
    pub marker: Bullet,
    pub position: Position,
    pub velocity: Velocity,
    pub mass: Mass,
    pub passive: Passive,
}

impl BulletBundle {
    fn new(position: Position, direction: Vec2) -> Self {
        Self {
            name: Name::new("Bullet"),
            marker: Bullet,
            velocity: Velocity(direction * BULLET_SPEED),
            mass: Mass(BULLET_MASS),
            passive: Passive,
            position,
        }
    }
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (shoot_bullet.run_if(not(is_in_rollback)), check_collisions),
        );
    }
}

fn check_collisions(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Position), With<Bullet>>,
    planet_query: Query<(&Position, &Radius)>,
) {
    for (bullet, bullet_position) in bullet_query.iter() {
        for (planet_position, planet_radius) in planet_query.iter() {
            let distance = bullet_position.distance(planet_position.0) - planet_radius.0 as f32;
            if distance <= 0.0 {
                commands.entity(bullet).despawn();
            }
        }
    }
}

fn shoot_bullet(
    mut commands: Commands,
    tick_manager: Res<TickManager>,
    identity: NetworkIdentity,
    mut query: Query<
        (&Player, &Position, &mut ActionState<PlayerAction>),
        Or<(With<client::Predicted>, With<ReplicationTarget>)>,
    >,
) {
    let _tick = tick_manager.tick();
    for (player, position, action) in query.iter_mut() {
        if let Some(axis_data) = action.dual_axis_data(&PlayerAction::Shoot) {
            if axis_data.update_pair.length() > 0.8 {
                let bullet = BulletBundle::new(position.clone(), axis_data.update_pair);
                println!("Pew");
                if identity.is_server() {
                    commands.spawn((
                        bullet,
                        PreSpawnedPlayerObject::default(),
                        server::Replicate {
                            sync: server::SyncTarget {
                                // the bullet is predicted for the client who shot it
                                prediction: NetworkTarget::Single(player.0),
                                // the bullet is interpolated for other clients
                                interpolation: NetworkTarget::AllExceptSingle(player.0),
                            },
                            // NOTE: all predicted entities need to have the same replication group
                            group: PLAYER_REPLICATION_GROUP,
                            ..default()
                        },
                    ));
                } else {
                    commands.spawn((bullet, PreSpawnedPlayerObject::default()));
                }
            }
        }
    }
}
