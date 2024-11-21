use bevy::prelude::*;

#[derive(Component, Reflect, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Clone, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub struct AnimatedSpritePlugin;

impl Plugin for AnimatedSpritePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AnimationIndices>()
            .add_systems(Update, animate_sprite);
    }
}
