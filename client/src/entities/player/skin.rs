use std::collections::HashMap;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use lightyear::prelude::client::Predicted;
use robot_rumble_common::entities::player::{Player, PlayerSkin};

use crate::utils::spritesheet;

#[derive(serde::Deserialize, Asset, TypePath, Deref)]
struct SkinsConfig(HashMap<String, AnimationsConfig>);

#[derive(serde::Deserialize)]
struct AnimationsConfig {
    idle: Animation,
    run: Animation,
    jump: Animation,
    fall: Animation,
}

#[derive(serde::Deserialize)]
struct Animation {
    rows: u32,
    columns: u32,
    spritesheet: String,
    frame_duration: f32,
}

#[derive(Resource)]
struct SkinConfigHandle(pub Handle<SkinsConfig>);

#[derive(Component)]
struct SkinAnimationsHandle {
    pub idle: Handle<Image>,
    pub run: Handle<Image>,
    pub jump: Handle<Image>,
    pub fall: Handle<Image>,
}

pub struct SkinPlugin;
impl Plugin for SkinPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<SkinsConfig>::new(&[]))
            .add_plugins(spritesheet::AnimatedSpritePlugin)
            .add_systems(Startup, load_skin_config)
            .add_systems(Update, load_skin_on_player);
    }
}

fn load_skin_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading skins config");
    let skin_config: Handle<SkinsConfig> = asset_server.load("config/skins.ron");
    commands.insert_resource(SkinConfigHandle(skin_config));
}

fn load_skin_on_player(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<(Entity, &PlayerSkin), (Added<Predicted>, With<Player>)>,
    config_handle: Res<SkinConfigHandle>,
    skins_config: Res<Assets<SkinsConfig>>,
    asset_server: Res<AssetServer>,
) {
    for (player_entity, player_skin) in query.iter() {
        info!("Loading skin animations");
        if let Some(skin_config) = skins_config.get(config_handle.0.id()) {
            if let Some(skin) = skin_config.get(&player_skin.0) {
                let animations_handle = SkinAnimationsHandle {
                    idle: asset_server.load(&skin.idle.spritesheet),
                    run: asset_server.load(&skin.run.spritesheet),
                    jump: asset_server.load(&skin.jump.spritesheet),
                    fall: asset_server.load(&skin.fall.spritesheet),
                };
                let target_anim = &skin.idle;

                let layout = TextureAtlasLayout::from_grid(
                    UVec2::splat(32),
                    target_anim.columns,
                    target_anim.rows,
                    None,
                    None,
                );
                let texture_atlas_layout = texture_atlas_layouts.add(layout);
                let indices = spritesheet::AnimationIndices {
                    first: 0,
                    last: (target_anim.columns * target_anim.rows - 1) as usize,
                };
                let timer = spritesheet::AnimationTimer(Timer::from_seconds(
                    target_anim.frame_duration,
                    TimerMode::Repeating,
                ));
                commands.entity(player_entity).insert((
                    SpriteBundle {
                        transform: Transform::from_scale(Vec3::splat(3.0)),
                        texture: animations_handle.idle.clone(),
                        ..default()
                    },
                    TextureAtlas {
                        layout: texture_atlas_layout,
                        index: indices.first,
                    },
                    indices,
                    timer,
                    animations_handle,
                ));
            } else {
                warn!("Received invalid player skin id");
            };
        } else {
            warn!("Skin config not loaded yet");
        }
    }
}
