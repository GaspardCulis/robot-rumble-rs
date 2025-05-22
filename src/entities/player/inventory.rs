use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, GgrsSchedule};
use leafwing_input_manager::prelude::ActionState;

use crate::core::physics;

use super::{Player, PlayerAction, Weapon, weapons};

const DEFAULT_ARSENAL: [weapons::WeaponType; 3] = [
    weapons::WeaponType::Pistol,
    weapons::WeaponType::Shotgun,
    weapons::WeaponType::Rifle,
];

#[derive(Component, Debug, Reflect)]
pub struct Arsenal(pub Vec<(weapons::WeaponType, Entity)>);

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Arsenal>()
            .add_systems(Update, spawn_arsenal)
            .add_systems(
                GgrsSchedule,
                (handle_slot_change_inputs, summon_current_weapon)
                    .chain()
                    .in_set(physics::PhysicsSet::Player)
                    .before(super::update_weapon),
            );
    }
}

fn spawn_arsenal(mut commands: Commands, query: Query<Entity, (With<Player>, Without<Arsenal>)>) {
    for player_entity in query.iter() {
        let arsenal = Arsenal(
            DEFAULT_ARSENAL
                .iter()
                .map(|weapon_type| {
                    let weapon_entity = commands.spawn(weapon_type.clone()).add_rollback().id();

                    (weapon_type.clone(), weapon_entity)
                })
                .collect(),
        );

        let current_weapon = Weapon(arsenal.0.first().expect("Arsenal should not be empty").1);

        commands
            .entity(player_entity)
            .insert((arsenal, current_weapon));
    }
}

fn summon_current_weapon(
    mut commands: Commands,
    owner_query: Query<&Weapon, Changed<Weapon>>,
    weapon_query: Query<Entity, Without<physics::Position>>,
) {
    for weapon_ref in owner_query.iter() {
        if let Ok(weapon_entity) = weapon_query.get(weapon_ref.0) {
            commands
                .entity(weapon_entity)
                .insert(physics::PhysicsBundle::default());
        }
    }
}

fn handle_slot_change_inputs(
    mut commands: Commands,
    mut query: Query<(&mut Weapon, &Arsenal, &ActionState<PlayerAction>)>,
) {
    for (mut current_weapon, arsenal, action_state) in query.iter_mut() {
        for action in action_state.get_pressed() {
            let selected_slot = match action {
                PlayerAction::Slot1 => Some(1),
                PlayerAction::Slot2 => Some(2),
                PlayerAction::Slot3 => Some(3),
                _ => None,
            };

            if let Some(slot) = selected_slot {
                if let Some((_, selected_weapon)) = arsenal.0.get(slot - 1) {
                    if &current_weapon.0 != selected_weapon {
                        commands
                            .entity(current_weapon.0)
                            .remove::<physics::PhysicsBundle>();

                        current_weapon.0 = *selected_weapon;
                    }
                } else {
                    warn!("Weapon slot overflow");
                }
            }
        }
    }
}
