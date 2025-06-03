use std::{
    fs::File,
    path::{Path, PathBuf},
};

use bevy::prelude::*;

use crate::{
    core::physics::Position,
    entities::{
        planet::{self, Radius},
        satellite,
    },
};

#[derive(Event)]
pub struct LoadLevelSaveEvent {
    pub path: PathBuf,
}

pub struct LevelSavePlugin;
impl Plugin for LevelSavePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadLevelSaveEvent>()
            .add_systems(Update, handle_load_level_save_events);
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LevelSave {
    pub planets: Vec<PlanetSave>,
    pub satellites: Vec<SatelliteSave>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlanetSave {
    pub position: Vec2,
    pub radius: u32,
    pub r#type: planet::PlanetType,
    pub seed: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SatelliteSave {
    pub position: Vec2,
    pub kind: satellite::SatelliteKind,
}

fn handle_load_level_save_events(
    mut events: EventReader<LoadLevelSaveEvent>,
    mut planet_spawn_events: EventWriter<planet::SpawnPlanetEvent>,
    mut satellite_spawn_events: EventWriter<satellite::SpawnSatelliteEvent>,
) -> Result {
    for event in events.read() {
        let save = LevelSave::load(&event.path)?;

        for PlanetSave {
            position,
            radius,
            r#type,
            seed,
        } in save.planets
        {
            planet_spawn_events.write(planet::SpawnPlanetEvent {
                position: Position(position),
                radius: Radius(radius),
                r#type,
                seed,
            });
        }

        for SatelliteSave { position, kind } in save.satellites {
            satellite_spawn_events.write(satellite::SpawnSatelliteEvent {
                position: Position(position),
                scale: 0.7,
                kind,
            });
        }
    }

    Ok(())
}

impl LevelSave {
    pub fn save<P>(&self, path: P) -> Result<(), ron::Error>
    where
        P: AsRef<Path>,
    {
        let f = File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)
            .expect("Failed opening file");

        ron::Options::default().to_writer_pretty(f, &self, ron::ser::PrettyConfig::new())?;

        Ok(())
    }

    pub fn load<P>(path: P) -> Result<Self, ron::Error>
    where
        P: AsRef<Path>,
    {
        let f = File::open(path)?;
        let save: LevelSave = ron::de::from_reader(f)?;

        Ok(save)
    }
}
