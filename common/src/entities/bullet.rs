use bevy::prelude::*;
use client::is_in_rollback;
use leafwing_input_manager::prelude::ActionState;
use lightyear::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

pub const BULLET_SPEED: f32 = 1200.;
pub const BULLET_MASS: u32 = 20;

use crate::core::{
    gravity::{Mass, Passive},
    physics::{Position, Velocity},
};

use super::{
    planet::Radius,
    player::{Player, PlayerAction},
};

#[derive(Component, Debug, Reflect, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bullet;

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
    bullet_query: Query<
        (Entity, &Position),
        (
            With<Bullet>,
            Or<(
                // move predicted bullets
                With<client::Predicted>,
                // move server entities
                With<ReplicationTarget>,
                // move prespawned bullets
                With<PreSpawnedPlayerObject>,
            )>,
        ),
    >,
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
        (
            &Player,
            &Position,
            &Velocity,
            &mut ActionState<PlayerAction>,
        ),
        Or<(With<client::Predicted>, With<ReplicationTarget>)>,
    >,
) {
    let tick = tick_manager.tick();
    for (player, player_position, player_velocity, action) in query.iter_mut() {
        if let Some(axis_data) = action.dual_axis_data(&PlayerAction::Shoot) {
            if axis_data.update_pair.length() > 0.8 {
                let mut rng = StdRng::seed_from_u64(tick.0 as u64);
                let random_angle = Vec2::from_angle(rng.gen_range(-0.04..0.04));

                let bullet = (
                    Bullet,
                    Name::new("Bullet"),
                    Position(player_position.0),
                    Velocity(
                        axis_data.update_pair.rotate(random_angle) * BULLET_SPEED
                            + player_velocity.0,
                    ),
                    Mass(BULLET_MASS),
                    Passive,
                );

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
