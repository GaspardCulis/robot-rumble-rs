use bevy::prelude::*;

use crate::entities::planet::SpawnPlanetEvent;

pub struct WorldgenPlugin;
impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GenerateWorldEvent>()
            .add_systems(Update, handle_genworld_event);
    }
}

#[derive(Event)]
pub struct GenerateWorldEvent;

fn handle_genworld_event(
    mut events: EventReader<GenerateWorldEvent>,
    mut planet_sapwn_events: EventWriter<SpawnPlanetEvent>,
) {
    for _ in events.read() {
        todo!()
    }
}
