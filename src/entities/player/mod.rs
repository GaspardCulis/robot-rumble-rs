use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;
use leafwing_input_manager::prelude::*;

use crate::core::gravity::{Mass, Passive};
use crate::core::physics::{PhysicsSet, Position, Rotation, Velocity};
use crate::utils::math;

use super::planet;

mod animation;
mod skin;

// TODO: Move to config file
pub const PLAYER_MASS: u32 = 800;
pub const PLAYER_VELOCITY: f32 = 600.;
pub const PLAYER_RADIUS: f32 = 16. * 2.;

#[derive(Component, Clone, Debug, PartialEq)]
#[require(Visibility)]
pub struct Player {
    pub handle: usize,
}

#[derive(Component, Clone, Debug, PartialEq, Reflect, Deref)]
pub struct PlayerInputVelocity(Vec2);

#[derive(Actionlike, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect)]
pub enum PlayerAction {
    Jump,
    Sneak,
    Left,
    Right,
}

#[derive(Component, Clone, Debug, PartialEq, Reflect)]
pub struct PlayerSkin(pub String);

#[derive(Component, Clone, Debug, PartialEq, Reflect)]
pub struct InAir(bool);

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    marker: Player,
    position: Position,
    velocity: Velocity,
    rotation: Rotation,
    in_air: InAir,
    input_velocity: PlayerInputVelocity,
    action_state: ActionState<PlayerAction>,
    mass: Mass,
    passive: Passive,
}
impl PlayerBundle {
    pub fn new(handle: usize, position: Position) -> Self {
        Self {
            position,
            name: Name::new("Player"),
            marker: Player { handle },
            velocity: Velocity(Vec2::ZERO),
            rotation: Rotation(0.),
            in_air: InAir(true),
            input_velocity: PlayerInputVelocity(Vec2::ZERO),
            action_state: ActionState::default(),
            mass: Mass(PLAYER_MASS),
            passive: Passive,
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_plugins(skin::SkinPlugin)
            .add_plugins(animation::PlayerAnimationPlugin)
            .add_systems(
                GgrsSchedule,
                (player_physics, player_movement)
                    .chain()
                    .in_set(PhysicsSet::Player),
            );
    }
}

fn player_movement(
    mut query: Query<
        (
            &ActionState<PlayerAction>,
            &mut Velocity,
            &mut PlayerInputVelocity,
            &Rotation,
            &InAir,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (action_state, mut velocity, mut input_velocity, rotation, in_air) in query.iter_mut() {
        if action_state.pressed(&PlayerAction::Jump) && !in_air.0 {
            velocity.0 = Vec2::from_angle(rotation.0).rotate(Vec2::Y) * PLAYER_VELOCITY * 2.;
        }

        if action_state.pressed(&PlayerAction::Right) {
            input_velocity.0.x = math::lerp(input_velocity.0.x, PLAYER_VELOCITY, delta * 2.);
        }
        if action_state.pressed(&PlayerAction::Left) {
            input_velocity.0.x = math::lerp(input_velocity.0.x, -PLAYER_VELOCITY, delta * 2.);
        }

        if !(action_state.pressed(&PlayerAction::Right)
            || action_state.pressed(&PlayerAction::Left))
        {
            let mut slow_down_rate = 6.;
            if in_air.0 {
                slow_down_rate = 1.;
            }
            input_velocity.0.x = math::lerp(input_velocity.0.x, 0., delta * slow_down_rate);
        }

        if action_state.pressed(&PlayerAction::Sneak) {
            input_velocity.0.y = math::lerp(input_velocity.0.y, -PLAYER_VELOCITY, delta);
        } else {
            input_velocity.0.y = math::lerp(input_velocity.0.y, 0., delta * 10.);
        }
    }
}

pub fn player_physics(
    mut player_query: Query<
        (
            &mut InAir,
            &mut Position,
            &mut Rotation,
            &mut Velocity,
            &PlayerInputVelocity,
        ),
        (With<Player>, Without<planet::Planet>),
    >,
    planet_query: Query<(&Position, &planet::Radius), With<planet::Planet>>,
    time: Res<Time>,
) {
    for (mut in_air, mut player_position, mut player_rotation, mut velocity, input_velocity) in
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
        let mut short_angle = (target_angle - player_rotation.0) % math::RAD;
        short_angle = (2. * short_angle) % math::RAD - short_angle;
        player_rotation.0 += short_angle * time.delta_secs() * 6.;

        player_position.0 +=
            Vec2::from_angle(player_rotation.0).rotate(input_velocity.0) * time.delta_secs();

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
                let velocity_along_normal = velocity.0.dot(collision_normal);
                let reflexion_vector = velocity.0 - 2. * velocity_along_normal * collision_normal;
                velocity.0 = reflexion_vector * 0.5;
            } else {
                // Reset velocity
                velocity.0 = Vec2::ZERO;
            }

            in_air.0 = false;
        } else {
            in_air.0 = true;
        }
    }
}
