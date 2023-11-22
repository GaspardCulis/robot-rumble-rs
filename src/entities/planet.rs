use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::core::{gravity::Mass, physics::Position, spritesheet};

#[derive(Component)]
pub struct Planet;

#[derive(Component)]
pub struct Radius(pub u32);

#[derive(AssetCollection, Resource)]
struct PlanetAssets {
    #[asset(texture_atlas(tile_size_x = 100., tile_size_y = 100., columns = 25, rows = 8))]
    #[asset(path = "img/planet1.png")]
    planet: Handle<TextureAtlas>,
}

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

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<PlanetAssets>()
            .add_systems(Startup, spawn_planet);
    }
}

fn spawn_planet(mut commands: Commands, sprite: Res<PlanetAssets>) {
    let animation_indices = spritesheet::AnimationIndices {
        first: 0,
        last: 199,
    };
    let animation_timer =
        spritesheet::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating));

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: sprite.planet.clone(),
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
