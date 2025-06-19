use bevy::{ecs::system::SystemState, platform::collections::HashMap, prelude::*};
use bevy_asset_loader::prelude::*;
use std::time::Duration;

use crate::{core::physics::Position, utils::spritesheet};

use super::{Player, PlayerSkin};

pub const PLAYER_SKIN_SCALE: f32 = 2.4;
pub const PLAYER_SKIN_ZINDEX: f32 = 10.0;

#[derive(AssetCollection, Resource, Clone)]
/// Temporary assets holding skin config only
pub struct SkinConfigAssets {
    #[asset(path = "config/skins", collection(mapped, typed))]
    skins: HashMap<AssetFileStem, Handle<SkinConfig>>,
}

#[derive(serde::Deserialize, Asset, TypePath, Clone, Debug)]
/// Skin config format
pub struct SkinConfig {
    pub idle: AnimationConfig,
    pub run: AnimationConfig,
    pub jump: AnimationConfig,
    pub fall: AnimationConfig,
}

#[derive(serde::Deserialize, Reflect, Clone, Debug)]
/// Config format for a specific Skin state
pub struct AnimationConfig {
    pub rows: u32,
    pub columns: u32,
    pub spritesheet: String,
    pub frame_duration: f32,
}

#[derive(Resource, Reflect)]
/// The actual loaded skin assets. Key is the skin file name stem.
pub struct SkinAssets {
    skins: HashMap<String, Handle<Skin>>,
}

#[derive(Asset, Reflect)]
/// The asset holding all the skin informations
pub struct Skin {
    pub idle: SkinAnimation,
    pub run: SkinAnimation,
    pub jump: SkinAnimation,
    pub fall: SkinAnimation,
}

#[derive(Reflect)]
/// Holds a specific skin state informations
pub struct SkinAnimation {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
    pub indices: spritesheet::AnimationIndices,
    pub timer: spritesheet::AnimationTimer,
    pub duration: Duration,
}

#[derive(Component, Debug, Reflect)]
/// Component holding a reference to a specific Skin asset
pub struct SkinHandle(pub Handle<Skin>);

pub struct SkinPlugin;
impl Plugin for SkinPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Skin>()
            .add_plugins(spritesheet::AnimatedSpritePlugin)
            .add_systems(
                Update,
                (
                    load_skin_on_player.run_if(resource_exists::<SkinAssets>),
                    #[cfg(feature = "dev_tools")]
                    handle_config_reload,
                ),
            );
    }
}

fn load_skin_on_player(
    mut commands: Commands,
    query: Query<(Entity, &PlayerSkin, &Position), (With<Player>, Without<Sprite>)>,
    assets: Res<SkinAssets>,
    skins: Res<Assets<Skin>>,
) {
    for (player_entity, player_skin, player_position) in query.iter() {
        info!("Loading skin animations for {:?}", player_entity);
        if let Some(skin_handle) = assets.skins.get(&player_skin.0) {
            if let Some(skin) = skins.get(skin_handle) {
                let default_anim = &skin.idle;

                commands.entity(player_entity).insert((
                    Sprite::from_atlas_image(
                        default_anim.texture.clone(),
                        TextureAtlas {
                            layout: default_anim.atlas_layout.clone(),
                            index: default_anim.indices.first,
                        },
                    ),
                    Transform::from_scale(Vec3::splat(PLAYER_SKIN_SCALE)).with_translation(
                        Vec3::new(player_position.0.x, player_position.0.y, PLAYER_SKIN_ZINDEX),
                    ),
                    SkinHandle(skin_handle.clone()),
                    default_anim.indices.clone(),
                    default_anim.timer.clone(),
                ));
            } else {
                error!("Skin config not loaded yet, should not happen!");
            };
        } else {
            warn!("Received invalid player skin id: {}", player_skin.0);
        }
    }
}

#[cfg(feature = "dev_tools")]
fn handle_config_reload(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<Skin>>,
    players: Query<Entity, (With<Player>, With<PlayerSkin>, With<Sprite>)>,
) {
    for event in events.read() {
        if let AssetEvent::Modified { id: _ } = event {
            for player in players.iter() {
                commands.entity(player).remove::<Sprite>();
            }
        };
    }
}

impl FromWorld for SkinAssets {
    fn from_world(world: &mut World) -> Self {
        let mut system_state = SystemState::<(
            ResMut<Assets<TextureAtlasLayout>>,
            Res<Assets<SkinConfig>>,
            Res<SkinConfigAssets>,
            Res<AssetServer>,
        )>::new(world);
        let (mut layouts, configs, assets, asset_server) = system_state.get_mut(world);

        let skins = assets
            .skins
            .iter()
            .map(|(name, skin)| {
                let config = configs.get(skin).unwrap();

                let skin = Skin {
                    idle: config.idle.build(&asset_server, &mut layouts),
                    run: config.run.build(&asset_server, &mut layouts),
                    jump: config.jump.build(&asset_server, &mut layouts),
                    fall: config.fall.build(&asset_server, &mut layouts),
                };

                (name.clone().into(), asset_server.add(skin))
            })
            .collect();

        Self { skins }
    }
}

impl AnimationConfig {
    fn build(
        &self,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> SkinAnimation {
        let texture = asset_server.load(&self.spritesheet);
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

        SkinAnimation {
            texture,
            atlas_layout,
            indices,
            timer,
            duration,
        }
    }
}
