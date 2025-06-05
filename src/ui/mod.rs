use bevy::{prelude::*, ui::{widget::NodeImageMode}};
use leafwing_input_manager::prelude::ActionState;

use crate::{entities::player::{weapon::{self, WeaponState, WeaponStats}, Player, PlayerAction, Weapon}, GameState};


const WEAPON_SLOTS: [weapon::WeaponType; 3] = [
    weapon::WeaponType::Pistol,
    weapon::WeaponType::Shotgun,
    weapon::WeaponType::Rifle,
];

#[derive(Component)]
struct WeaponSlotUI {
    index: usize,
}

#[derive(Component)]
struct AmmoText;

#[derive(Component)]
struct SelectedWeaponSlot;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            spawn_arsenal_hud,
        )
        .add_systems(
            Update,
            (update_weapon_slot_ui.run_if(in_state(GameState::InGame)),
            update_ammo_text.run_if(in_state(GameState::InGame))),
        );
    }
}

fn spawn_arsenal_hud(
    mut commands: Commands,
    config_handle: Res<weapon::WeaponsConfigHandle>,
    config_assets: Res<Assets<weapon::config::WeaponsConfig>>,
    asset_server: Res<AssetServer>,
) {
    let weapons_config = if let Some(c) = config_assets.get(config_handle.0.id()) {
        c
    } else {
        warn!("Couldn't load WeaponsConfig");
        return;
    };

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            justify_content: JustifyContent::End,
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        border: UiRect::all(Val::Px(2.0)),
                        padding: UiRect::all(Val::Px(8.0)),
                        row_gap: Val::Px(6.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                    BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
                ))
                .with_children(|column| {
                    for (i, weapon_type) in WEAPON_SLOTS.iter().enumerate() {
                        let weapon_config = weapons_config
                            .0
                            .get(weapon_type)
                            .expect("Missing weapon config");
                        let skin = weapon_config.skin.clone();
                        let base_size = 48.0;
                        let size = base_size * skin.scale;

                        column
                            .spawn(Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(8.0),
                                ..default()
                            })
                            .with_children(|row| {
                                row.spawn((
                                    Text::new(format!("{}", i + 1)),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                    TextLayout {
                                        justify: JustifyText::Center,
                                        ..default()
                                    },
                                ));

                                let is_selected = i == 0; // arme par défaut sélectionnée (slot 1)

                                if is_selected {
                                    row
                                        .spawn((
                                            Node {
                                                display: Display::Flex,
                                                flex_direction: FlexDirection::Column,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                width: Val::Px(96.0),
                                                height: Val::Px(96.0),
                                                margin: UiRect::all(Val::Px(4.0)),
                                                padding: UiRect::all(Val::Px(6.0)),
                                                border: UiRect::all(Val::Px(1.0)),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                                            BorderColor(Color::WHITE),
                                            WeaponSlotUI { index: i },
                                            SelectedWeaponSlot,
                                        ))
                                        .with_children(|slot| {
                                            slot.spawn((
                                                Node {
                                                    width: Val::Px(size),
                                                    height: Val::Px(size),
                                                    ..default()
                                                },
                                                ImageNode {
                                                    image: asset_server.load(skin.sprite.clone()),
                                                    image_mode: NodeImageMode::Stretch,
                                                    ..default()
                                                },
                                            ));
                                        });
                                } else {
                                    row
                                        .spawn((
                                            Node {
                                                display: Display::Flex,
                                                flex_direction: FlexDirection::Column,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                width: Val::Px(96.0),
                                                height: Val::Px(96.0),
                                                margin: UiRect::all(Val::Px(4.0)),
                                                padding: UiRect::all(Val::Px(6.0)),
                                                border: UiRect::all(Val::Px(1.0)),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                                            BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.1)),
                                            WeaponSlotUI { index: i },
                                        ))
                                        .with_children(|slot| {
                                            slot.spawn((
                                                Node {
                                                    width: Val::Px(size),
                                                    height: Val::Px(size),
                                                    ..default()
                                                },
                                                ImageNode {
                                                    image: asset_server.load(skin.sprite.clone()),
                                                    image_mode: NodeImageMode::Stretch,
                                                    ..default()
                                                },
                                            ));
                                        });
                                }
                            });
                    }
                });
        });
        commands
            .spawn(Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                bottom: Val::Px(350.0),
                right: Val::Px(16.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            })
            .with_children(|builder| {
                builder.spawn((
                    Text::new("Ammo: 0 / 0"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout {
                        justify: JustifyText::Left,
                        ..default()
                    },
                    AmmoText,
                ));
            });

}

fn update_weapon_slot_ui(
    mut commands: Commands,
    mut query_ui: Query<(Entity, &WeaponSlotUI, Option<&SelectedWeaponSlot>, &mut BorderColor)>,
    query_input: Query<&ActionState<PlayerAction>>,
) {
    let Some(input) = query_input.iter().next() else { return; };

    let selected_index = if input.just_pressed(&PlayerAction::Slot1) {
        Some(0)
    } else if input.just_pressed(&PlayerAction::Slot2) {
        Some(1)
    } else if input.just_pressed(&PlayerAction::Slot3) {
        Some(2)
    } else {
        None
    };

    if let Some(new_selected) = selected_index {
        for (entity, slot_ui, selected_marker, mut border_color) in query_ui.iter_mut() {
            if slot_ui.index == new_selected {
                // Nouveau slot sélectionné
                if selected_marker.is_none() {
                    commands.entity(entity).insert(SelectedWeaponSlot);
                }
                *border_color = BorderColor(Color::WHITE);
            } else {
                // Les autres redeviennent normaux
                if selected_marker.is_some() {
                    commands.entity(entity).remove::<SelectedWeaponSlot>();
                }
                *border_color = BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.1));
            }
        }
    }
}

fn update_ammo_text(
    weapon_query: Query<&Weapon, With<Player>>,
    weapon_state_query: Query<(&WeaponState, &WeaponStats)>,
    mut text_query: Query<&mut Text, With<AmmoText>>,
) {
    if let Ok(weapon) = weapon_query.single() {
        if let Ok((state, stats)) = weapon_state_query.get(weapon.0) {
            if let Ok(mut text) = text_query.single_mut() {
                text.0 = format!("Ammo: {} / {}", state.current_ammo, stats.magazine_size);
            }
        }
    }
}
