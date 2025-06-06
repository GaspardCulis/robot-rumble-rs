use bevy::{prelude::*, ui::widget::NodeImageMode};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    GameState,
    entities::player::{
        Player, PlayerAction, Weapon,
        weapon::{self, WeaponState, WeaponStats},
    },
};

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

#[derive(Component)]
struct AmmoBackground;

#[derive(Component)]
struct BlackBar;

#[derive(Component)]
pub struct AmmoReloadAnimation {
    pub from: f32,
    pub to: f32,
    pub timer: Timer,
}


pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_arsenal_hud)
            .add_systems(
                Update,
                (
                    update_weapon_slot_ui.run_if(in_state(GameState::InGame)),
                    update_ammo_text.run_if(in_state(GameState::InGame)),
                    update_ammo_background.run_if(in_state(GameState::InGame)),
                    trigger_reload_animation.run_if(in_state(GameState::InGame)),
                    animate_ammo_reload.run_if(in_state(GameState::InGame)),
                ),
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

    // Conteneur principal avec position relative
    commands
        .spawn(Node {
            position_type: PositionType::Relative,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            justify_content: JustifyContent::End,
            ..default()
        })
        .with_children(|parent| {
            // Le fond (bande noire + blanche)
            parent
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(150.0),
                    height: Val::Px(350.0),
                    ..default()
                })
                .with_children(|background| {
                    // Bande noire (haut)
                    background.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(0.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                        BlackBar
                    ));

                    // Bande blanche (bas)
                    background.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
                        AmmoBackground
                    ));
                });

            // Le HUD superposé sur le fond
            parent
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    width: Val::Px(150.0),
                    height: Val::Px(350.0),
                    bottom: Val::Px(0.0),
                    right: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    row_gap: Val::Px(6.0),
                    ..default()
                })
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

                                let is_selected = i == 0;

                                let mut slot_entity = row.spawn((
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
                                    BorderColor(if is_selected {
                                        Color::WHITE
                                    } else {
                                        Color::srgba(1.0, 1.0, 1.0, 0.1)
                                    }),
                                    WeaponSlotUI { index: i },
                                ));
                                if is_selected {
                                    slot_entity.insert(SelectedWeaponSlot);
                                }
                                slot_entity.with_children(|slot| {
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
                            });
                    }
                });

            // HUD secondaire : munitions
            parent
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
        });
}


fn update_weapon_slot_ui(
    mut commands: Commands,
    mut query_ui: Query<(
        Entity,
        &WeaponSlotUI,
        Option<&SelectedWeaponSlot>,
        &mut BorderColor,
    )>,
    query_input: Query<&ActionState<PlayerAction>>,
) {
    let Some(input) = query_input.iter().next() else {
        return;
    };

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

fn update_ammo_background(
    weapon_query: Query<&Weapon, With<Player>>,
    weapon_state_query: Query<(&WeaponState, &WeaponStats)>,
    mut background_query: Query<(Entity, &mut Node), (With<AmmoBackground>, Without<BlackBar>)>,
    mut black_bar_query: Query<&mut Node, (With<BlackBar>, Without<AmmoBackground>)>,
    reload_anim_query: Query<&AmmoReloadAnimation>, // Ajouté ici
) {
    let Ok(weapon) = weapon_query.single() else { return; };
    let Ok((state, stats)) = weapon_state_query.get(weapon.0) else { return; };
    let Ok((entity, mut node)) = background_query.single_mut() else { return; };

    // ✅ Ne pas mettre à jour si une animation est en cours
    if reload_anim_query.get(entity).is_ok() {
        return;
    }

    let percent = if stats.magazine_size > 0 {
        state.current_ammo as f32 / stats.magazine_size as f32
    } else {
        0.0
    }.clamp(0.0, 1.0);

    node.height = Val::Percent(percent * 100.0);

    if let Ok(mut black_bar) = black_bar_query.single_mut() {
        black_bar.height = Val::Percent((1.0 - percent) * 100.0);
    }
}



fn trigger_reload_animation(
    weapon_query: Query<&Weapon, With<Player>>,
    weapon_state_query: Query<(&WeaponState, &WeaponStats)>,
    mut background_query: Query<(Entity, &mut Node), With<AmmoBackground>>,
    input_query: Query<&ActionState<PlayerAction>>,
    mut commands: Commands,
    reload_anim_query: Query<&AmmoReloadAnimation>,
) {
    let Ok(weapon) = weapon_query.single_inner() else { return; };
    let Ok((state, stats)) = weapon_state_query.get(weapon.0) else { return; };
    let Ok((entity, node)) = background_query.single_mut() else { return; };

    // Ignore si une animation est en cours
    if reload_anim_query.get(entity).is_ok() {
        return;
    }

    let Ok(input) = input_query.single() else { return; };

    let reload_pressed = input.just_pressed(&PlayerAction::Reload);

    // Trigger reload animation if player presses reload OR has 0 ammo
    if reload_pressed || state.current_ammo == 0 {

        let from = match node.height {
            Val::Percent(p) => p / 100.0,
            _ => 0.0,
        };
        let to = 1.0;

        let duration = stats.reload_time; // ou stats.reload_time selon ton modèle

        commands.entity(entity).insert(AmmoReloadAnimation {
            from,
            to,
            timer: Timer::from_seconds(duration.as_secs_f32(), TimerMode::Once),
        });

    }
}


fn animate_ammo_reload(
    time: Res<Time>,
    mut query: Query<(&mut AmmoReloadAnimation, &mut Node, Entity), With<AmmoBackground>>,
    mut black_bar_query: Query<&mut Node, (With<BlackBar>, Without<AmmoBackground>)>,
    mut commands: Commands,
) {
    for (mut anim, mut node, entity) in query.iter_mut() {

        anim.timer.tick(time.delta());
        let progress = (anim.timer.elapsed_secs() / anim.timer.duration().as_secs_f32()).clamp(0.0, 1.0);
        let percent = anim.from + (anim.to - anim.from) * progress;
        node.height = Val::Percent(percent * 100.0);

        // Update black bar accordingly
        if let Ok(mut black_bar) = black_bar_query.single_mut() {
            black_bar.height = Val::Percent((1.0 - percent) * 100.0);
        }

        // Optionally, remove the animation component when done
        if anim.timer.finished() {
            node.height = Val::Percent(anim.to * 100.0);
            if let Ok(mut black_bar) = black_bar_query.single_mut() {
                black_bar.height = Val::Percent((1.0 - anim.to) * 100.0);
            }
            // Supprimer le composant à la fin de l'animation
            commands.entity(entity).remove::<AmmoReloadAnimation>();
        }
    }
}
