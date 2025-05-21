use bevy::prelude::*;

use super::{Player, weapons};

#[derive(Component, Debug, Reflect)]
pub struct Arsenal(pub Vec<weapons::WeaponType>);

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Arsenal>()
            .register_required_components::<Player, Arsenal>();
    }
}

impl Default for Arsenal {
    fn default() -> Self {
        Self(vec![
            weapons::WeaponType::Pistol,
            weapons::WeaponType::Shotgun,
            weapons::WeaponType::Rifle,
        ])
    }
}
