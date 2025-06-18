use bevy::platform::collections::HashMap;
use bevy::prelude::AudioSource;
use bevy::prelude::*;
use bevy::prelude::{App, Plugin};

/// A generic key for named audio clips
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum SoundEffect {
    Shooting,
}

#[derive(Event)]
pub struct SoundEvent {
    pub clip: SoundEffect,
}

#[derive(Resource, Default)]
pub struct GameAudio {
    clips: HashMap<SoundEffect, Handle<AudioSource>>,
}

impl GameAudio {
    pub fn get(&self, clip: &SoundEffect) -> Option<&Handle<AudioSource>> {
        self.clips.get(clip)
    }
}
/// Plugin that loads audio assets and provides a playback API.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>()
            .insert_resource(GameAudio::default())
            .add_systems(Startup, load_audio_assets)
            .add_systems(Update, play_sound);
    }
}

/// Load audio files and map them to `AudioClip` keys.
fn load_audio_assets(asset_server: Res<AssetServer>, mut audio: ResMut<GameAudio>) {
    audio
        .clips
        .insert(SoundEffect::Shooting, asset_server.load("audio/fire.ogg"));
}

fn play_sound(
    mut commands: Commands,
    mut events: EventReader<SoundEvent>,
    game_audio: Res<GameAudio>,
) {
    for SoundEvent { clip } in events.read() {
        if let Some(sound) = game_audio.get(clip) {
            commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
        }
    }
}
