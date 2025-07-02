use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct GameAudioPlugin;

// Channel for SFX audio
#[derive(Resource)]
pub struct AudioSFX;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin).add_audio_channel::<AudioSFX>();
    }
}
