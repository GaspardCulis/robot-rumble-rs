use bevy::prelude::*;

use super::materials::*;

#[derive(Bundle)]
pub struct EarthLike {
    ocean: PlanetMaterialLayerInit<UnderMaterial>,
    land: PlanetMaterialLayerInit<LandmassesMaterial>,
    clouds: PlanetMaterialLayerInit<CloudsMaterial>,
}

impl EarthLike {
    pub fn new() -> Self {
        Self {
            ocean: PlanetMaterialLayerInit {
                material: Default::default(),
                scale: 1.,
                z_index: -2.,
            },
            land: PlanetMaterialLayerInit {
                material: Default::default(),
                scale: 1.,
                z_index: -1.,
            },
            clouds: PlanetMaterialLayerInit {
                material: Default::default(),
                scale: 1.,
                z_index: 0.,
            },
        }
    }
}
