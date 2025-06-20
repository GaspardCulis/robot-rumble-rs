use bevy::prelude::*;
use bevy::prelude::{App, Plugin};
use bevy_kira_audio::AudioSource;
use bevy_kira_audio::prelude::*;

#[allow(dead_code)] // Temporarly until sound effects are passed by events
#[derive(Event)]
pub struct SoundEvent {
    pub handle: Handle<AudioSource>,
}

pub struct GameAudioPlugin;

// Channel for SFX audio
#[derive(Resource)]
pub struct AudioSFX;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>()
            .add_plugins(AudioPlugin)
            .add_audio_channel::<AudioSFX>();
    }
}

#[allow(dead_code)] // Temporarly until sound is completely managed by audio module
fn _play_sound(mut events: EventReader<SoundEvent>, sfx_channel: Res<AudioChannel<AudioSFX>>) {
    for SoundEvent { handle } in events.read() {
        sfx_channel.play(handle.clone());
    }
}
