use bevy::prelude::AudioSource;
use bevy::prelude::*;
use bevy::prelude::{App, Plugin};

#[derive(Event)]
pub struct SoundEvent {
    pub handle: Handle<AudioSource>,
}

/// Plugin that loads audio assets and provides a playback API.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>()
            .add_systems(Update, play_sound);
    }
}

fn play_sound(mut commands: Commands, mut events: EventReader<SoundEvent>) {
    for SoundEvent { handle } in events.read() {
        commands.spawn((AudioPlayer(handle.clone()), PlaybackSettings::DESPAWN));
    }
}
