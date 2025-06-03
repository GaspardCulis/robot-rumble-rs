use std::{fs::File, path::Path};

use crate::entities::{planet, satellite};
use bevy::math::Vec2;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SaveFile {
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

impl SaveFile {
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
        let save: SaveFile = ron::de::from_reader(f)?;

        Ok(save)
    }
}
