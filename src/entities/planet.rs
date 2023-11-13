use bevy::prelude::*;

use crate::core::spritesheet;

#[derive(Component)]
pub struct Planet;

pub fn spawn_planet(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = assets_server.load("img/planet1.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(100., 100.), 25, 8, None, None);
    let texture_alias_handle = texture_atlases.add(texture_atlas);

    let animation_indices = spritesheet::AnimationIndices {
        first: 0,
        last: 200,
    };
    let animation_timer =
        spritesheet::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating));

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_alias_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            ..default()
        },
        animation_indices,
        animation_timer,
    ));
}
