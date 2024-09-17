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

#[derive(Bundle)]
pub struct MoonLike {
    under: PlanetMaterialLayerInit<UnderMaterial>,
    craters: PlanetMaterialLayerInit<CratersMaterial>,
}

impl MoonLike {
    pub fn new() -> Self {
        Self {
            under: PlanetMaterialLayerInit {
                material: UnderMaterial {
                    common: CommonMaterial {
                        size: 8.,
                        light_origin: Vec2::new(0.25, 0.25),
                        ..Default::default()
                    },
                    color1: Srgba::hex("#a3a7c2").unwrap().into(),
                    color2: Srgba::hex("#4c6885").unwrap().into(),
                    color3: Srgba::hex("#3a3f5e").unwrap().into(),
                    light_border_1: 0.615,
                    light_border_2: 0.729,
                    ..Default::default()
                },
                scale: 1.,
                z_index: -1.,
            },

            craters: PlanetMaterialLayerInit {
                material: Default::default(),
                scale: 1.,
                z_index: 0.,
            },
        }
    }
}
