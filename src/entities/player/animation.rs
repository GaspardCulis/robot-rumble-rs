use super::{
    PlanetCollision, Player, PlayerAction,
    skin::{PLAYER_SKIN_SCALE, SkinAnimationsHandle},
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::utils::spritesheet;

#[derive(Component, Reflect, Default, PartialEq, Eq)]
enum PlayerAnimationState {
    #[default]
    Idle,
    Run,
    Jump(Timer),
    Fall,
}

pub struct PlayerAnimationPlugin;
impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerAnimationState>().add_systems(
            Update,
            (
                add_animation_state_on_player,
                update_sprite_texture,
                update_animation_state,
                update_orientation,
            )
                .chain(),
        );
    }
}

fn add_animation_state_on_player(mut commands: Commands, query: Query<Entity, Added<Player>>) {
    for player_entity in query.iter() {
        commands
            .entity(player_entity)
            .insert(PlayerAnimationState::default());
    }
}

fn update_sprite_texture(
    mut query: Query<
        (
            &PlayerAnimationState,
            &SkinAnimationsHandle,
            &mut Sprite,
            &mut spritesheet::AnimationIndices,
            &mut spritesheet::AnimationTimer,
        ),
        Changed<PlayerAnimationState>,
    >,
) {
    for (state, animations, mut sprite, mut indices, mut timer) in query.iter_mut() {
        let anim_handle = match state {
            PlayerAnimationState::Idle => &animations.idle,
            PlayerAnimationState::Run => &animations.run,
            PlayerAnimationState::Jump(_) => &animations.jump,
            PlayerAnimationState::Fall => &animations.fall,
        };
        sprite.image = anim_handle.texture.clone();
        *indices = anim_handle.indices.clone();
        *timer = anim_handle.timer.clone();
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.layout = anim_handle.atlas_layout.clone();
            atlas.index = 0;
        } else {
            warn!("No atlas attached to sprite");
        }
    }
}

fn update_animation_state(
    mut query: Query<
        (
            &mut PlayerAnimationState,
            &SkinAnimationsHandle,
            &ActionState<PlayerAction>,
            &PlanetCollision,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    for (mut state, anims, inputs, planet_collision) in query.iter_mut() {
        match state.bypass_change_detection() {
            PlayerAnimationState::Jump(timer) => {
                timer.tick(time.delta());
                if !timer.just_finished() {
                    // Let animation finish
                    continue;
                }
            }
            _ => (),
        };

        let new_state = if !planet_collision.collides {
            PlayerAnimationState::Fall
        } else if inputs.just_pressed(&PlayerAction::Jump) {
            let timer = Timer::new(anims.jump.duration, TimerMode::Once);
            PlayerAnimationState::Jump(timer)
        } else if inputs.pressed(&PlayerAction::Right) || inputs.pressed(&PlayerAction::Left) {
            PlayerAnimationState::Run
        } else {
            PlayerAnimationState::Idle
        };

        if new_state != *state {
            *state = new_state;
        }
    }
}

fn update_orientation(mut query: Query<(&mut Transform, &ActionState<PlayerAction>)>) {
    for (mut transform, inputs) in query.iter_mut() {
        if inputs.just_pressed(&PlayerAction::Right) {
            transform.scale.x = PLAYER_SKIN_SCALE;
        } else if inputs.just_pressed(&PlayerAction::Left) {
            transform.scale.x = -PLAYER_SKIN_SCALE;
        }
    }
}
