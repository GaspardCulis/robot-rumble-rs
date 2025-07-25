use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;
use leafwing_input_manager::prelude::*;
use weapon::{WeaponMode, WeaponState};

use crate::core::collision::{CollisionPlugin, CollisionShape, CollisionState};
use crate::core::gravity::{Mass, Passive};
use crate::core::inputs::{PlayerAction, PlayerActionState};
use crate::core::physics::{PhysicsSet, Position, Rotation, Velocity};
use crate::entities::player::weapon::config::WeaponStats;
use crate::utils::math;

use super::planet;
use crate::entities::satellite::slingshot::Orbited;

mod animation;
pub mod inventory;
pub mod skin;
pub mod weapon;

// TODO: Move to config file
pub const PLAYER_MASS: u32 = 800;
pub const PLAYER_VELOCITY: f32 = 600.;
pub const PLAYER_JUMP_VELOCITY: f32 = 800.;
pub const PLAYER_RADIUS: f32 = 16. * 2.;
const PLAYER_GROUND_FRICTION_COEFF: f32 = 0.95;

type PlanetCollision = CollisionState<Player, planet::Planet>;

#[derive(Component, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Reflect)]
#[require(
    // Position, Velocity and Rotation not required because handled by level::spawn.
    // Might change in the future
    Visibility,
    PlayerInputVelocity,
    Passive,
    ActionState<PlayerAction>,
    Mass(PLAYER_MASS),
    CollisionShape::Circle(PLAYER_RADIUS),
    PlayerSkin("laika.skin".into()),
    Name::new("Player"),
    Percentage::default(),
)]
pub struct Player {
    pub handle: usize,
}

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct Percentage(pub f32);

#[derive(Component, Clone, Debug, Default, PartialEq, Reflect, Deref)]
pub struct PlayerInputVelocity(Vec2);

#[derive(Component, Clone, Debug, PartialEq, Reflect)]
pub struct PlayerSkin(pub String);

#[derive(Component, Clone, Debug, PartialEq, Reflect)]
#[relationship(relationship_target = weapon::Owner)]
pub struct Weapon(pub Entity);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<PlayerInputVelocity>()
            .register_type::<PlayerSkin>()
            .register_type::<Weapon>()
            .register_type::<Percentage>()
            .add_plugins(CollisionPlugin::<Player, planet::Planet>::new())
            .add_plugins(animation::PlayerAnimationPlugin)
            .add_plugins(inventory::InventoryPlugin)
            .add_plugins(skin::SkinPlugin)
            .add_plugins(weapon::WeaponPlugin)
            .add_systems(
                GgrsSchedule,
                (player_physics, player_movement, update_weapon)
                    .chain()
                    .in_set(PhysicsSet::Player),
            );
    }
}

fn player_movement(
    mut query: Query<
        (
            &PlayerActionState,
            &mut Velocity,
            &mut PlayerInputVelocity,
            &Rotation,
            &PlanetCollision,
        ),
        (With<Player>, Without<Orbited>),
    >,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (action_state, mut velocity, mut input_velocity, rotation, collision) in query.iter_mut() {
        if !collision.collides {
            input_velocity.0 = input_velocity.lerp(Vec2::ZERO, delta * 6.);
            continue;
        }

        if action_state.pressed(&PlayerAction::Jump) {
            velocity.0 = Vec2::from_angle(rotation.0).rotate(Vec2::Y) * PLAYER_JUMP_VELOCITY;
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
            input_velocity.0.x = math::lerp(input_velocity.0.x, 0., delta * 6.0);
        }

        if action_state.pressed(&PlayerAction::Sneak) {
            input_velocity.0.y = math::lerp(input_velocity.0.y, -PLAYER_VELOCITY, delta);
        } else {
            input_velocity.0.y = math::lerp(input_velocity.0.y, 0., delta * 10.);
        }
    }
}

fn update_weapon(
    player_query: Query<(&ActionState<PlayerAction>, &Position, &Velocity, &Weapon), With<Player>>,
    mut weapon_query: Query<
        (
            &mut weapon::WeaponMode,
            &mut Position,
            &mut Velocity,
            &mut Rotation,
            &WeaponState,
            &WeaponStats,
        ),
        Without<Player>,
    >,
) {
    for (action_state, player_position, player_velocity, weapon) in player_query.iter() {
        let axis_pair = action_state.axis_pair(&PlayerAction::PointerDirection);
        if let Ok((
            mut mode,
            mut position,
            mut velocity,
            mut direction,
            weapon_state,
            weapon_stats,
        )) = weapon_query.get_mut(weapon.0)
        {
            direction.0 = if axis_pair != Vec2::ZERO {
                axis_pair.to_angle()
            } else {
                0.0
            };
            let pressed = action_state.get_pressed();
            if pressed.contains(&PlayerAction::Shoot) && weapon_state.current_ammo > 0 {
                *mode = WeaponMode::Triggered;
            } else if pressed.contains(&PlayerAction::Reload)
                && weapon_state.current_ammo < weapon_stats.magazine_size
                && *mode != WeaponMode::Reloading
            {
                *mode = WeaponMode::Reloading;
            } else if *mode != WeaponMode::Reloading {
                *mode = WeaponMode::Idle;
            }
            position.0 = player_position.0;
            velocity.0 = player_velocity.0;
        } else {
            warn!("Failed to retrieve weapon entity with ID {:?}", weapon.0);
        }
    }
}

fn player_physics(
    mut player_query: Query<
        (
            &mut Position,
            &mut Rotation,
            &mut Velocity,
            &PlanetCollision,
            &PlayerInputVelocity,
        ),
        (With<Player>, Without<planet::Planet>, Without<Orbited>),
    >,
    planet_query: Query<(&Position, &CollisionShape), With<planet::Planet>>,
    time: Res<Time>,
) {
    for (
        mut player_position,
        mut player_rotation,
        mut velocity,
        planet_collision,
        input_velocity,
    ) in player_query.iter_mut()
    {
        // Find nearest planet (asserts that one planet exists)
        let (nearest_planet_pos, nearest_planet_shape) = planet_collision
            .closest
            .and_then(|entity| planet_query.get(entity).ok())
            .map(|(pos, shape)| (pos.clone(), shape.clone()))
            .unwrap_or_default();

        // Rotate towards it
        let target_angle = bevy::math::ops::atan2(
            nearest_planet_pos.y - player_position.0.y,
            nearest_planet_pos.x - player_position.0.x,
        ) + PI / 2.;
        let mut short_angle = (target_angle - player_rotation.0) % math::RAD;
        short_angle = (2. * short_angle) % math::RAD - short_angle;
        player_rotation.0 += short_angle * time.delta_secs() * 6.;

        player_position.0 +=
            Vec2::from_angle(player_rotation.0).rotate(input_velocity.0) * time.delta_secs();

        // Check if collides
        if planet_collision.collides {
            // Compute collision normal
            let collision_normal = (player_position.0 - nearest_planet_pos.0).normalize();
            // Clip player to ground
            let clip_position = nearest_planet_pos.0
                + collision_normal * (PLAYER_RADIUS + nearest_planet_shape.bounding_radius());
            player_position.0 = clip_position;

            // Bounce if not on feet
            let rotation_diff = math::clip_angle(player_rotation.0 - target_angle);
            if rotation_diff.abs() > 30f32.to_radians() {
                let velocity_along_normal = velocity.0.dot(collision_normal);
                let reflexion_vector = velocity.0 - 2. * velocity_along_normal * collision_normal;
                velocity.0 = reflexion_vector * 0.5;
            } else {
                // Reset velocity along collision normal
                let dot_product = velocity.dot(collision_normal);
                velocity.0 -= dot_product * collision_normal;
                // Apply ground friction
                velocity.0 *= PLAYER_GROUND_FRICTION_COEFF * time.delta_secs();
            }
        }
    }
}
