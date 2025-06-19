use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, GgrsSchedule};
use leafwing_input_manager::prelude::ActionState;

use crate::core::physics;

use super::{Player, PlayerAction, Weapon, weapon::config::WeaponType};

const DEFAULT_ARSENAL: [WeaponType; 3] = [
    WeaponType::BlackholeGun,
    WeaponType::Shotgun,
    WeaponType::Rifle,
];

#[derive(Component, Debug, Reflect)]
pub struct Arsenal(pub Vec<(WeaponType, Entity)>);

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Arsenal>()
            .add_systems(Update, (spawn_arsenal, update_slot_inputs))
            .add_systems(
                GgrsSchedule,
                (handle_slot_change_inputs, summon_current_weapon)
                    .chain()
                    .in_set(physics::PhysicsSet::Player)
                    .before(super::update_weapon),
            );
    }
}

fn spawn_arsenal(mut commands: Commands, query: Query<(Entity, &Player), Without<Arsenal>>) {
    for (player_entity, _) in query
        .iter()
        //Sort by handle for determinism
        .sort::<&Player>()
    {
        let arsenal = Arsenal(
            DEFAULT_ARSENAL
                .iter()
                .map(|weapon_type| {
                    let weapon_entity = commands
                        .spawn(weapon_type.clone())
                        .insert(Visibility::Hidden)
                        .add_rollback()
                        .id();

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
                .insert(physics::PhysicsBundle::default())
                .insert(Visibility::Visible);
        }
    }
}

fn handle_slot_change_inputs(
    mut commands: Commands,
    query: Query<(Entity, &Weapon, &Arsenal, &ActionState<PlayerAction>)>,
) {
    for (player, current_weapon, arsenal, action_state) in query.iter() {
        for action in action_state.get_just_pressed() {
            let selected_slot = match action {
                PlayerAction::Slot1 => Some(0),
                PlayerAction::Slot2 => Some(1),
                PlayerAction::Slot3 => Some(2),
                _ => None,
            };

            if let Some(slot) = selected_slot {
                if let Some((_, selected_weapon)) = arsenal.0.get(slot) {
                    if &current_weapon.0 != selected_weapon {
                        // Hide current weapon
                        commands
                            .entity(current_weapon.0)
                            .remove::<physics::PhysicsBundle>()
                            .insert(Visibility::Hidden);

                        // Set selected weapon
                        // Cannot mutate relationship components so we replace the old one
                        commands.entity(player).insert(Weapon(*selected_weapon));
                    }
                } else {
                    warn!("Weapon slot overflow");
                }
            }
        }
    }
}

/// Updates `PlayerAction::SlotX` input presses based on `PlayerAction::SlotNext` and `PlayerAction::SlotPrev` to avoid aving to serialize these inputs.
fn update_slot_inputs(mut query: Query<(&mut ActionState<PlayerAction>, &Weapon, &Arsenal)>) {
    for (mut action_state, current_weapon, arsenal) in query.iter_mut() {
        for action in action_state.get_just_pressed() {
            if let Some(slot_offset) = match action {
                PlayerAction::SlotNext => Some(1),
                PlayerAction::SlotPrev => Some(-1),
                _ => None,
            } {
                let current_slot = arsenal
                    .0
                    .iter()
                    .position(|(_, weapon)| weapon == &current_weapon.0)
                    .unwrap_or_default();

                let selected_slot = if slot_offset > 0 {
                    (current_slot + 1) % arsenal.0.len()
                } else {
                    current_slot.checked_sub(1).unwrap_or(arsenal.0.len() - 1)
                };

                println!("Switching  from slot {current_slot} to {selected_slot}");

                let action = match selected_slot {
                    0 => PlayerAction::Slot1,
                    1 => PlayerAction::Slot2,
                    2 => PlayerAction::Slot3,
                    _ => unimplemented!("Should not happend"),
                };

                action_state.press(&action);
            }
        }
    }
}
