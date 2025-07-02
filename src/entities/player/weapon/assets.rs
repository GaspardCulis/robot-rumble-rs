use bevy::{ecs::system::SystemState, platform::collections::HashMap, prelude::*};

use super::config::{WeaponType, WeaponsConfig, WeaponsConfigAssets};

#[derive(Resource, Reflect, Deref)]
pub struct WeaponsAssets(HashMap<WeaponType, WeaponAssets>);

#[derive(Asset, Reflect)]
pub struct WeaponAssets {
    pub skin: Handle<Image>,
    pub fire: Handle<bevy_kira_audio::AudioSource>,
    pub reload: Option<Handle<bevy_kira_audio::AudioSource>>,
}

impl FromWorld for WeaponsAssets {
    fn from_world(world: &mut World) -> Self {
        let mut system_state = SystemState::<(
            Res<Assets<WeaponsConfig>>,
            Res<WeaponsConfigAssets>,
            Res<AssetServer>,
        )>::new(world);
        let (configs, assets, asset_server) = system_state.get_mut(world);

        let config = configs
            .get(&assets.config)
            .expect("WeaponsConfig should be loaded at this point");

        let weapon_assets = config
            .0
            .iter()
            .map(|(weapon_type, weapon_config)| {
                (
                    weapon_type.clone(),
                    WeaponAssets {
                        skin: asset_server.load(&weapon_config.skin.sprite),
                        fire: asset_server.load(&weapon_config.sounds.fire),
                        reload: weapon_config
                            .sounds
                            .reload
                            .as_ref()
                            .map(|reload| asset_server.load(reload)),
                    },
                )
            })
            .collect();

        Self(weapon_assets)
    }
}
