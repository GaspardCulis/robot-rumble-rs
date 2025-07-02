use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{core::audio::AudioSFX, entities::player::weapon::config::WeaponType};

use super::{WeaponEvent, assets::WeaponsAssets};

#[derive(Component, Reflect)]
struct AudioReload(Handle<AudioInstance>);

pub struct WeaponSFXPlugin;
impl Plugin for WeaponSFXPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_weapon_events.run_if(resource_exists::<WeaponsAssets>),
        );
    }
}

fn handle_weapon_events(
    mut commands: Commands,
    mut events: EventReader<WeaponEvent>,
    weapon_query: Query<(&WeaponType, Option<&AudioReload>)>,
    assets: Res<WeaponsAssets>,
    sfx_channel: Res<AudioChannel<AudioSFX>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) -> Result {
    for event in events.read() {
        let entity = event.get_entity();
        let (weapon_type, audio_reload) = weapon_query.get(entity)?;
        let weapon_assets = assets.get(weapon_type).ok_or(BevyError::from(format!(
            "Failed to retreive weapon assets for {weapon_type:?}",
        )))?;

        match event {
            WeaponEvent::Fire(_) => {
                sfx_channel.play(weapon_assets.fire.clone());
            }
            WeaponEvent::ReloadStart(_) => {
                if let Some(reload_sfx_handle) = weapon_assets.reload.clone()
                    && audio_reload.is_none()
                {
                    let handle = sfx_channel.play(reload_sfx_handle).handle();
                    commands.entity(entity).insert(AudioReload(handle));
                };
            }
            WeaponEvent::ReloadEnd(_) => {
                commands.entity(entity).remove::<AudioReload>();
            }
            WeaponEvent::Equipped(_) => {
                if let Some(instance) = audio_reload
                    .and_then(|audio_reload_handle| audio_instances.get_mut(&audio_reload_handle.0))
                {
                    instance.resume(AudioTween::default());
                }
            }
            WeaponEvent::UnEquipped(_) => {
                if let Some(instance) = audio_reload
                    .and_then(|audio_reload_handle| audio_instances.get_mut(&audio_reload_handle.0))
                {
                    instance.stop(AudioTween::default());
                }
            }
        };
    }

    Ok(())
}
