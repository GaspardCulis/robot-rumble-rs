use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        camera::CameraFollowTarget,
        gravity::{Mass, Passive},
        physics::{Position, Rotation, Velocity},
    },
    utils::math::{self, RAD},
};

use super::planet::{Planet, Radius};

const PLAYER_MASS: u32 = 800;
const PLAYER_VELOCITY: f32 = 600.;
const PLAYER_RADIUS: f32 = 16.;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Player(ClientId);

#[derive(Component, Debug, Reflect)]
pub struct PlayerInputVelocity(Vec2);

#[derive(Actionlike, Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect)]
pub enum PlayerAction {
    Jump,
    Sneak,
    Left,
    Right,
}

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(path = "img/player.png")]
    player: Handle<Image>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum PlayerState {
    #[default]
    InAir,
    OnGround,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    marker: Player,
    position: Position,
    velocity: Velocity,
    rotation: Rotation,
    mass: Mass,
    passive: Passive,
}

impl PlayerBundle {
    pub fn new(client_id: ClientId, position: Position) -> Self {
        Self {
            position,
            name: Name::new("Player"),
            marker: Player(client_id),
            velocity: Velocity(Vec2::X * 5.),
            rotation: Rotation(0.),
            mass: Mass(PLAYER_MASS),
            passive: Passive,
        }
    }
}

pub enum PlayerPlugin {
    Client,
    Server,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .add_systems(Update, player_physics);

        match self {
            PlayerPlugin::Client => {
                app.init_collection::<PlayerAssets>()
                    .register_type::<PlayerInputVelocity>()
                    .add_systems(Update, (insert_sprite, handle_keys));
            }
            PlayerPlugin::Server => (),
        };
    }
}

fn insert_sprite(
    mut commands: Commands,
    query: Query<Entity, Added<Player>>,
    sprite: Res<PlayerAssets>,
) {
    for entity in query.iter() {
        let mut entity_commands = commands.entity(entity);

        entity_commands.insert(SpriteBundle {
            texture: sprite.player.clone(),
            transform: Transform::from_scale(Vec3::splat(0.2)),
            ..default()
        });
    }

    /*
    TODO
    let input_map = InputMap::new([
        // Jump
        (PlayerAction::Jump, KeyCode::Space),
        (PlayerAction::Jump, KeyCode::KeyW),
        // Sneak
        (PlayerAction::Sneak, KeyCode::ShiftLeft),
        (PlayerAction::Sneak, KeyCode::KeyS),
        // Directions
        (PlayerAction::Right, KeyCode::KeyD),
        (PlayerAction::Left, KeyCode::KeyA),
    ]); + CameraFollowTarget*/
}

fn handle_keys(
    mut query: Query<(
        &mut Position,
        &mut PlayerInputVelocity,
        &mut Velocity,
        &Rotation,
        &ActionState<PlayerAction>,
    )>,
    player_state: Res<State<PlayerState>>,
    time: Res<Time>,
) {
    // TODO: Run system when specific state instead of checking
    if query.is_empty() {
        return;
    };

    let (mut position, mut input_velocity, mut velocity, rotation, action_state) =
        query.single_mut();
    let delta = time.delta_seconds();

    if action_state.pressed(&PlayerAction::Jump) && *player_state.get() == PlayerState::OnGround {
        velocity.0 = Vec2::from_angle(rotation.0).rotate(Vec2::Y) * PLAYER_VELOCITY * 2.;
        // Immediately update position
        position.0 += velocity.0 * delta;
    }

    if action_state.pressed(&PlayerAction::Right) {
        input_velocity.0.x = math::lerp(input_velocity.0.x, PLAYER_VELOCITY, delta * 2.);
    }
    if action_state.pressed(&PlayerAction::Left) {
        input_velocity.0.x = math::lerp(input_velocity.0.x, -PLAYER_VELOCITY, delta * 2.);
    }

    if !(action_state.pressed(&PlayerAction::Right) || action_state.pressed(&PlayerAction::Left)) {
        let mut slow_down_rate = 6.;
        if *player_state.get() == PlayerState::InAir {
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

fn player_physics(
    mut player_query: Query<(&mut Position, &mut Rotation, &mut Velocity), Without<Planet>>,
    planet_query: Query<(&Position, &Radius), With<Planet>>,
    mut player_state: ResMut<NextState<PlayerState>>,
    time: Res<Time>,
) {
    for (mut player_position, mut player_rotation, mut velocity) in player_query.iter_mut() {
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

            player_state.set(PlayerState::OnGround);
        } else {
            player_state.set(PlayerState::InAir);
        }
    }
}
