use std::f64::consts::PI;

use bevy::prelude::*;

use crate::core::{gravity::Mass, physics::Position, spritesheet};

#[derive(Component)]
pub struct Planet;

#[derive(Component)]
pub struct Radius(pub u32);

#[derive(Bundle)]
struct PlanetBundle {
    marker: Planet,
    position: Position,
    radius: Radius,
    mass: Mass,
}

impl Default for PlanetBundle {
    fn default() -> Self {
        const DEFAULT_RADIUS: u32 = 512;
        Self {
            marker: Planet,
            position: Position(Vec2::ZERO),
            radius: Radius(radius_to_mass(DEFAULT_RADIUS)),
            mass: Mass(DEFAULT_RADIUS),
        }
    }
}

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
        last: 199,
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
        PlanetBundle {
            mass: Mass(1200),
            ..Default::default()
        },
    ));
}

fn radius_to_mass(radius: u32) -> u32 {
    (PI * radius.pow(2) as f64) as u32
}
