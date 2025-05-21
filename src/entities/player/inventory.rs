use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension as _, GgrsSchedule};
use leafwing_input_manager::prelude::ActionState;

use crate::core::physics::{self, PhysicsSet};

use super::{Player, PlayerAction, Weapon, weapons};

#[derive(Component, Debug, Reflect)]
pub struct Arsenal(pub Vec<weapons::WeaponType>);

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Arsenal>()
            .register_required_components::<Player, Arsenal>()
            .add_systems(
                GgrsSchedule,
                (load_default_weapon, handle_slot_change_inputs)
                    .chain()
                    .in_set(PhysicsSet::Player)
                    .before(super::update_weapon),
            );
    }
}

fn load_default_weapon(mut commands: Commands, query: Query<(Entity, &Arsenal), Without<Weapon>>) {
    for (entity, arsenal) in query.iter() {
        if let Some(default_weapon_type) = arsenal.0.first().cloned() {
            let weapon = commands
                .spawn((
                    default_weapon_type,
                    physics::Position(Vec2::ZERO),
                    physics::Velocity(Vec2::ZERO),
                    physics::Rotation(0.0),
                ))
                .id();

            commands.entity(entity).insert(Weapon(weapon));
        } else {
            warn!("Received empty arsenal");
        }
    }
}

fn handle_slot_change_inputs(
    mut commands: Commands,
    mut query: Query<(&mut Weapon, &Arsenal, &ActionState<PlayerAction>)>,
    weapon_query: Query<&weapons::WeaponType>,
) {
    for (mut weapon_entity, arsenal, action_state) in query.iter_mut() {
        for action in action_state.get_pressed() {
            let selected_slot = match action {
                PlayerAction::Slot1 => Some(1),
                PlayerAction::Slot2 => Some(2),
                PlayerAction::Slot3 => Some(3),
                _ => None,
            };

            if let Some(slot) = selected_slot {
                if let Some(selected_weapon_type) = arsenal.0.get(slot - 1) {
                    if let Ok(current_weapon_type) = weapon_query.get(weapon_entity.0) {
                        if selected_weapon_type != current_weapon_type {
                            // FIX: State is reset so we can bypass reload time
                            let new_weapon = commands
                                .spawn((
                                    selected_weapon_type.clone(),
                                    physics::Position(Vec2::ZERO),
                                    physics::Velocity(Vec2::ZERO),
                                    physics::Rotation(0.0),
                                ))
                                .add_rollback()
                                .id();
                            // Despawn old weapon
                            commands.entity(weapon_entity.0).despawn();
                            // Update player weapon pointer
                            weapon_entity.0 = new_weapon;
                        }
                    } else {
                        warn!(
                            "Player Weapon component holds a reference to an invalid Weapon entity"
                        );
                    }
                } else {
                    warn!("Weapon slot overflow");
                }
            }
        }
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
