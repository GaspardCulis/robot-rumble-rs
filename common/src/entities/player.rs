use std::f32::consts::PI;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::{connection::id::ClientId, prelude::Tick};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        gravity::{Mass, Passive},
        physics::{PhysicsSet, Position, Rotation, Velocity},
    },
    utils::math::{self, RAD},
};

use super::planet::{Planet, Radius};

// TODO: Move to config file
pub const PLAYER_MASS: u32 = 800;
pub const PLAYER_VELOCITY: f32 = 600.;
pub const PLAYER_RADIUS: f32 = 16. * 2.;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Player(ClientId);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect, Deref)]
pub struct PlayerInputVelocity(Vec2);

#[derive(Actionlike, Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect)]
pub enum PlayerAction {
    Jump,
    Sneak,
    Left,
    Right,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct PlayerSkin(pub String);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct InAir;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    marker: Player,
    position: Position,
    velocity: Velocity,
    rotation: Rotation,
    input_velocity: PlayerInputVelocity,
    mass: Mass,
    passive: Passive,
}

impl PlayerBundle {
    pub fn new(client_id: ClientId, position: Position) -> Self {
        Self {
            position,
            name: Name::new("Player"),
            marker: Player(client_id),
            velocity: Velocity(Vec2::ZERO),
            rotation: Rotation(0.),
            input_velocity: PlayerInputVelocity(Vec2::ZERO),
            mass: Mass(PLAYER_MASS),
            passive: Passive,
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(FixedUpdate, PlayerSet.after(PhysicsSet))
            .add_systems(FixedUpdate, player_physics.in_set(PlayerSet));
    }
}

#[derive(bevy::ecs::query::QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct SharedApplyInputsQuery {
    player: &'static Player,
    position: &'static mut Position,
    velocity: &'static mut Velocity,
    input_velocity: &'static mut PlayerInputVelocity,
    rotation: &'static Rotation,
    in_air: Has<InAir>,
}

pub fn movement_behaviour(
    action_state: &ActionState<PlayerAction>,
    saiq: &mut SharedApplyInputsQueryItem,
    _tick: Tick,
) {
    let delta = 1. / 60.;

    if action_state.pressed(&PlayerAction::Jump) && !saiq.in_air {
        saiq.velocity.0 = Vec2::from_angle(saiq.rotation.0).rotate(Vec2::Y) * PLAYER_VELOCITY * 2.;
        // Immediately update position
        saiq.position.0 += saiq.velocity.0 * delta;
    }

    if action_state.pressed(&PlayerAction::Right) {
        saiq.input_velocity.0.x = math::lerp(saiq.input_velocity.0.x, PLAYER_VELOCITY, delta * 2.);
    }
    if action_state.pressed(&PlayerAction::Left) {
        saiq.input_velocity.0.x = math::lerp(saiq.input_velocity.0.x, -PLAYER_VELOCITY, delta * 2.);
    }

    if !(action_state.pressed(&PlayerAction::Right) || action_state.pressed(&PlayerAction::Left)) {
        let mut slow_down_rate = 6.;
        if saiq.in_air {
            slow_down_rate = 1.;
        }
        saiq.input_velocity.0.x = math::lerp(saiq.input_velocity.0.x, 0., delta * slow_down_rate);
    }

    if action_state.pressed(&PlayerAction::Sneak) {
        saiq.input_velocity.0.y = math::lerp(saiq.input_velocity.0.y, -PLAYER_VELOCITY, delta);
    } else {
        saiq.input_velocity.0.y = math::lerp(saiq.input_velocity.0.y, 0., delta * 10.);
    }
}

pub fn player_physics(
    mut commands: Commands,
    mut player_query: Query<
        (
            Entity,
            &mut Position,
            &mut Rotation,
            &mut Velocity,
            &PlayerInputVelocity,
        ),
        (With<Player>, Without<Planet>),
    >,
    planet_query: Query<(&Position, &Radius), With<Planet>>,
    time: Res<Time>,
) {
    for (player_entity, mut player_position, mut player_rotation, mut velocity, input_velocity) in
        player_query.iter_mut()
    {
        // Find nearest planet (asserts that one planet exists)
        let mut nearest_position = Vec2::ZERO;
        let mut nearest_radius: u32 = 0;
        let mut nearest_distance = f32::MAX;
        for (position, radius) in planet_query.iter() {
            let distance = position.0.distance(player_position.0) - radius.0 as f32;
            if distance < nearest_distance {
                nearest_position = position.0;
                nearest_radius = radius.0;
                nearest_distance = distance;
            }
        }

        // Rotate towards it
        let target_angle = (nearest_position.y - player_position.0.y)
            .atan2(nearest_position.x - player_position.0.x)
            + PI / 2.;
        let mut short_angle = (target_angle - player_rotation.0) % RAD;
        short_angle = (2. * short_angle) % RAD - short_angle;
        player_rotation.0 += short_angle * time.delta_seconds() * 6.;

        player_position.0 +=
            Vec2::from_angle(player_rotation.0).rotate(input_velocity.0) * time.delta_seconds();

        // Check if collides
        if nearest_distance - PLAYER_RADIUS <= 0. {
            // Compute collision normal
            let collision_normal = (player_position.0 - nearest_position).normalize();
            // Clip player to ground
            let clip_position =
                nearest_position + collision_normal * (PLAYER_RADIUS + nearest_radius as f32);
            player_position.0 = clip_position;

            // Bounce if not on feet
            let rotation_diff = math::clip_angle(player_rotation.0 - target_angle);
            if rotation_diff.abs() > 30f32.to_radians() {
                println!("Bouncing");
                let velocity_along_normal = velocity.0.dot(collision_normal);
                let reflexion_vector = velocity.0 - 2. * velocity_along_normal * collision_normal;
                velocity.0 = reflexion_vector * 0.5;
            } else {
                // Reset velocity
                velocity.0 = Vec2::ZERO;
            }

            commands.entity(player_entity).remove::<InAir>();
        } else {
            commands.entity(player_entity).insert(InAir);
        }
    }
}

// Chore ops implementations

impl std::ops::Add for PlayerInputVelocity {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl std::ops::Mul<f32> for &PlayerInputVelocity {
    type Output = PlayerInputVelocity;

    fn mul(self, rhs: f32) -> Self::Output {
        PlayerInputVelocity(self.0 * rhs)
    }
}
