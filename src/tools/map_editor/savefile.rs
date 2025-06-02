use std::{fs::File, path::Path};

use bevy::math::Vec2;
use robot_rumble::entities::planet;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SaveFile {
    pub planets: Vec<PlanetSave>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlanetSave {
    pub position: Vec2,
    pub radius: u32,
    pub r#type: planet::PlanetType,
    pub seed: u64,
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
}
