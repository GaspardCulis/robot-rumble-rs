use bevy::prelude::*;

use super::weapons;

#[derive(Component, Debug, Reflect)]
pub struct Arsenal(pub Vec<weapons::WeaponType>);
