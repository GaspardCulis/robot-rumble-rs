use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use std::{collections::HashMap, time::Duration};

use crate::utils::spritesheet;

use super::{Player, PlayerSkin};

pub const PLAYER_SKIN_SCALE: f32 = 3.;

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
pub struct SkinAnimationsHandle {
    pub idle: AnimationHandle,
    pub run: AnimationHandle,
    pub jump: AnimationHandle,
    pub fall: AnimationHandle,
}

pub struct AnimationHandle {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
    pub indices: spritesheet::AnimationIndices,
    pub timer: spritesheet::AnimationTimer,
    pub duration: Duration,
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
    query: Query<(Entity, &PlayerSkin), Added<Player>>,
    config_handle: Res<SkinConfigHandle>,
    skins_config: Res<Assets<SkinsConfig>>,
    asset_server: Res<AssetServer>,
) {
    for (player_entity, player_skin) in query.iter() {
        info!("Loading skin animations for {:?}", player_entity);
        if let Some(skin_config) = skins_config.get(config_handle.0.id()) {
            if let Some(skin) = skin_config.get(&player_skin.0) {
                let animations_handle = SkinAnimationsHandle {
                    idle: skin
                        .idle
                        .build_handle(&asset_server, &mut texture_atlas_layouts),
                    run: skin
                        .run
                        .build_handle(&asset_server, &mut texture_atlas_layouts),
                    jump: skin
                        .jump
                        .build_handle(&asset_server, &mut texture_atlas_layouts),
                    fall: skin
                        .fall
                        .build_handle(&asset_server, &mut texture_atlas_layouts),
                };
                let default_anim = &animations_handle.idle;

                commands.entity(player_entity).insert((
                    Sprite::from_atlas_image(
                        default_anim.texture.clone(),
                        TextureAtlas {
                            layout: default_anim.atlas_layout.clone(),
                            index: default_anim.indices.first,
                        },
                    ),
                    Transform::from_scale(Vec3::splat(PLAYER_SKIN_SCALE)),
                    default_anim.indices.clone(),
                    default_anim.timer.clone(),
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

impl Animation {
    fn build_handle(
        &self,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> AnimationHandle {
        let texture = asset_server.load(self.spritesheet.clone());
        let layout =
            TextureAtlasLayout::from_grid(UVec2::splat(32), self.columns, self.rows, None, None);
        let atlas_layout = texture_atlas_layouts.add(layout);
        let indices = spritesheet::AnimationIndices {
            first: 0,
            last: (self.columns * self.rows - 1) as usize,
        };
        let timer = spritesheet::AnimationTimer(Timer::from_seconds(
            self.frame_duration,
            TimerMode::Repeating,
        ));
        let duration =
            Duration::from_secs_f32(self.frame_duration * (self.columns * self.rows) as f32);

        AnimationHandle {
            texture,
            atlas_layout,
            indices,
            timer,
            duration,
        }
    }
}
