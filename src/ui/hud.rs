use bevy::{prelude::*, ui::widget::NodeImageMode};

use crate::{
    core::inputs::{PlayerAction, PlayerActionState}, entities::player::{
        inventory::Arsenal, weapon::{
            config::{WeaponStats, WeaponType, WeaponsAssets, WeaponsConfig}, WeaponState
        }, Player, Weapon
    }, GameState
};


#[derive(Component)]
struct WeaponSlotUI {
    weapon_type: WeaponType,
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
    pub original_weapon_entity: Entity,
}

#[derive(Component)]
struct WeaponNameBoxUI {
    weapon_type: WeaponType,
}

#[derive(Component)]
struct WeaponSpriteUI {
    weapon_type: WeaponType,
}

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_arsenal_hud,
                update_weapon_slot_ui,
                update_ammo_text,
                update_ammo_background,
                trigger_reload_animation,
                animate_ammo_reload,
            )
                .run_if(in_state(GameState::InGame)),
            );
    }
}

fn spawn_arsenal_hud(
    mut commands: Commands,
    weapon_assets: Res<WeaponsAssets>,
    config_assets: Res<Assets<WeaponsConfig>>,
    asset_server: Res<AssetServer>,
    query_players: Query<(Entity, &Player, &Arsenal), Changed<Arsenal>>,
) {
    let weapons_config = if let Some(c) = config_assets.get(&weapon_assets.config) {
        c
    } else {
        warn!("Couldn't load WeaponsConfig");
        return;
    };

    for (_entity, _player, maybe_arsenal) in query_players.iter() {
        let arsenal: Vec<WeaponType> = maybe_arsenal
            .0
            .iter()
            .map(|(weapon_type, _)| weapon_type.clone())
            .collect();

        info!("Spawning HUD for arsenal: {:?}", maybe_arsenal);

        commands
            .spawn(Node {
                position_type: PositionType::Relative,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                bottom: Val::Px(28.0),
                align_items: AlignItems::End,
                justify_content: JustifyContent::End,
                ..default()
            })
            .with_children(|parent| {
                // Fond (bande noire + blanche)
                parent
                    .spawn(Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        width: Val::Px(381.0),
                        height: Val::Px(122.0),
                        ..default()
                    })
                    .with_children(|background| {
                        background.spawn((
                            Node {
                                width: Val::Percent(0.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                            BlackBar,
                        ));
                        background.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
                            AmmoBackground,
                        ));
                    });

                // HUD principal
                parent
                    .spawn(Node {
                        position_type: PositionType::Absolute,
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(350.0),
                        height: Val::Px(150.0),
                        bottom: Val::Px(-28.0),
                        right: Val::Px(50.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        ..default()
                    })
                    .with_children(|column| {
                        // Slot d'arme sélectionnée
                        column
                            .spawn(Node {
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
                            })
                            .with_children(|frame| {
                                frame.spawn((
                                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                                    BorderColor(Color::WHITE),
                                    SelectedWeaponSlot,
                                    WeaponSlotUI { weapon_type: arsenal[0].clone() },
                                ));

                                for (i, weapon_type) in arsenal.iter().enumerate() {
                                    if let Some(config) = weapons_config.0.get(weapon_type) {
                                        let skin = &config.skin;
                                        let size = 96.0 * skin.scale;
                                        let mut image_node = frame.spawn((
                                            Node {
                                                width: Val::Px(size),
                                                height: Val::Px(size),
                                                display: if i == 0 {
                                                    Display::Flex
                                                } else {
                                                    Display::None
                                                },
                                                ..default()
                                            },
                                            ImageNode {
                                                image: asset_server.load(skin.sprite.clone()),
                                                image_mode: NodeImageMode::Stretch,
                                                ..default()
                                            },
                                            WeaponSpriteUI { weapon_type: weapon_type.clone() },
                                        ));
                                        if i == 0 {
                                            image_node.insert(SelectedWeaponSlot);
                                        }
                                    }
                                }
                            });

                        // Noms des armes
                        column
                            .spawn(Node {
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(0.0),
                                right: Val::Px(-50.0),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                ..default()
                            })
                            .with_children(|row| {
                                for (i, weapon_type) in arsenal.iter().enumerate() {
                                    let formatted_name =
                                        format_weapon_name(&format!("{weapon_type:?}"));
                                    let is_selected = i == 0;
                                    row.spawn((
                                        Node {
                                            width: Val::Px(127.0),
                                            height: Val::Px(28.0),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        BackgroundColor(if is_selected {
                                            Color::srgba(1.0, 1.0, 1.0, 0.2)
                                        } else {
                                            Color::srgba(0.0, 0.0, 0.0, 0.6)
                                        }),
                                        WeaponNameBoxUI { weapon_type: weapon_type.clone() },
                                    ))
                                    .with_children(|name_box| {
                                        name_box.spawn((
                                            Text::new(format!("{}  {}", i + 1, formatted_name)),
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                            TextLayout {
                                                justify: JustifyText::Center,
                                                ..default()
                                            },
                                        ));
                                    });
                                }
                            });
                    });

                // Munitions
                parent
                    .spawn(Node {
                        position_type: PositionType::Absolute,
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        bottom: Val::Px(60.0),
                        right: Val::Px(16.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn((
                            Text::new("0 / 0"),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                            TextLayout {
                                justify: JustifyText::Left,
                                ..default()
                            },
                            AmmoText,
                        ));
                    });
            });

        // Si tu veux HUD pour un seul joueur, décommente cette ligne :
        // break;
    }
}


// Fonction pour formater le nom de l'arme
fn format_weapon_name(weapon_type: &str) -> String {
    weapon_type
        .replace("_", " ") // Remplace les underscores par des espaces
        .to_lowercase()
        .split_whitespace()
        .map(|word| {
            // Met en majuscule la première lettre de chaque mot
            word.chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_uppercase().collect::<String>()
                    } else {
                        c.to_string()
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ") // Rejoint les mots avec un espace
}

fn update_weapon_slot_ui(
    mut commands: Commands,
    mut query_ui: Query<(
        Entity,
        &WeaponSlotUI,
        Option<&SelectedWeaponSlot>,
        &mut BorderColor,
    )>,
    mut name_boxes: Query<(&mut BackgroundColor, &WeaponNameBoxUI)>,
    mut weapon_sprites: Query<(&mut Node, &WeaponSpriteUI), Without<AmmoBackground>>,
    weapon_query: Query<&Weapon, With<Player>>,
    weapon_state_query: Query<(&WeaponState, &WeaponStats)>,
    mut background_query: Query<(Entity, &mut Node), With<AmmoBackground>>,
    reload_anim_query: Query<Option<&AmmoReloadAnimation>>,
    test_query: Query<(&Weapon, &Player, &Arsenal), Changed<Weapon>>,
) {


    let mut current_weapon_type: Option<WeaponType> = None;

    if let Ok(weapon) = weapon_query.single() {
        if let Some((_weapon, _player, arsenal)) = test_query.iter().next() {
            for (weapon_type, weapon_entity) in &arsenal.0 {
                if *weapon_entity == weapon.0 {
                    current_weapon_type = Some(weapon_type.clone());
                    break;
                }
            }
        }
    }
    if let Some(current_weapon_type) = current_weapon_type.clone() {
        for (entity, slot_ui, selected_marker, mut border_color) in query_ui.iter_mut() {
            if slot_ui.weapon_type == current_weapon_type {
                if selected_marker.is_none() {
                    commands.entity(entity).insert(SelectedWeaponSlot);
                }
                *border_color = BorderColor(Color::WHITE);
            } else {
                if selected_marker.is_some() {
                    commands.entity(entity).remove::<SelectedWeaponSlot>();
                }
                *border_color = BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.1));
            }
        }

        for (mut bg_color, name_slot) in name_boxes.iter_mut() {
            if name_slot.weapon_type == current_weapon_type {
                *bg_color = BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.2));
            } else {
                *bg_color = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6));
            }
        }

        for (mut node, sprite_ui) in weapon_sprites.iter_mut() {
            node.display = if sprite_ui.weapon_type == current_weapon_type {
                Display::Flex
            } else {
                Display::None
            };
        }

        // Relancer l’animation si l’arme sélectionnée est en cours de rechargement
        if let Ok(weapon) = weapon_query.single()
            && let Ok((state, stats)) = weapon_state_query.get(weapon.0)
            && state.reload_timer.remaining_secs() > 0.0
            && let Ok((entity, _)) = background_query.single_mut()
            && reload_anim_query.get(entity).ok().flatten().is_none()
        {
            let reload_duration = stats.reload_time.as_secs_f32();
            let remaining_secs = state
                .reload_timer
                .remaining_secs()
                .clamp(0.0, reload_duration);
            let elapsed_ratio = 1.0 - (remaining_secs / reload_duration);
            let ammo_ratio = state.current_ammo as f32 / stats.magazine_size as f32;
            let from = ammo_ratio.max(elapsed_ratio);

            commands.entity(entity).insert(AmmoReloadAnimation {
                from,
                to: 1.0,
                timer: Timer::from_seconds(remaining_secs, TimerMode::Once),
                original_weapon_entity: weapon.0,
            });
        }
    }
}



fn update_ammo_text(
    weapon_query: Query<&Weapon, With<Player>>,
    weapon_state_query: Query<(&WeaponState, &WeaponStats)>,
    mut text_query: Query<&mut Text, With<AmmoText>>,
) {
    if let Ok(weapon) = weapon_query.single()
        && let Ok((state, stats)) = weapon_state_query.get(weapon.0)
        && let Ok(mut text) = text_query.single_mut()
    {
        text.0 = format!("{} / {}", state.current_ammo, stats.magazine_size);
    }
}

fn update_ammo_background(
    weapon_query: Query<&Weapon, With<Player>>,
    weapon_state_query: Query<(&WeaponState, &WeaponStats)>,
    mut background_query: Query<(Entity, &mut Node), (With<AmmoBackground>, Without<BlackBar>)>,
    mut black_bar_query: Query<&mut Node, (With<BlackBar>, Without<AmmoBackground>)>,
    reload_anim_query: Query<&AmmoReloadAnimation>,
) {
    let Ok(weapon) = weapon_query.single() else {
        return;
    };
    let Ok((state, stats)) = weapon_state_query.get(weapon.0) else {
        return;
    };
    let Ok((entity, mut node)) = background_query.single_mut() else {
        return;
    };

    // Ne pas mettre à jour si une animation est en cours
    if reload_anim_query.get(entity).is_ok() {
        return;
    }

    let percent = if stats.magazine_size > 0 {
        state.current_ammo as f32 / stats.magazine_size as f32
    } else {
        0.0
    }
    .clamp(0.0, 1.0);

    node.width = Val::Percent(percent * 100.0);

    if let Ok(mut black_bar) = black_bar_query.single_mut() {
        black_bar.width = Val::Percent((1.0 - percent) * 100.0);
    }
}

fn trigger_reload_animation(
    weapon_query: Query<&Weapon, With<Player>>,
    weapon_state_query: Query<(&WeaponState, &WeaponStats)>,
    mut background_query: Query<Entity, With<AmmoBackground>>,
    input_query: Query<&PlayerActionState>,
    mut commands: Commands,
    reload_anim_query: Query<&AmmoReloadAnimation>,
) {
    let Ok(weapon) = weapon_query.single_inner() else {
        return;
    };
    let Ok((state, stats)) = weapon_state_query.get(weapon.0) else {
        return;
    };
    let Ok(entity) = background_query.single_mut() else {
        return;
    };

    // Ignore si une animation est en cours
    if reload_anim_query.get(entity).is_ok() {
        return;
    }

    let Ok(input) = input_query.single() else {
        return;
    };

    let reload_pressed = input.just_pressed(&PlayerAction::Reload);

    // Trigger reload animation if player presses reload OR has 0 ammo
    if reload_pressed || state.current_ammo == 0 {
        let reload_duration = stats.reload_time.as_secs_f32();
        let remaining_secs = state
            .reload_timer
            .remaining_secs()
            .clamp(0.0, reload_duration);

        let elapsed_ratio = 1.0 - (remaining_secs / reload_duration);
        let ammo_ratio = state.current_ammo as f32 / stats.magazine_size as f32;

        // On utilise le plus grand des deux : l’avancée visuelle ne doit jamais être en retard par rapport aux munitions
        let from = ammo_ratio.max(elapsed_ratio);
        let to = 1.0;

        commands.entity(entity).insert(AmmoReloadAnimation {
            from,
            to,
            timer: Timer::from_seconds(remaining_secs, TimerMode::Once),
            original_weapon_entity: weapon.0,
        });
    }
}

fn animate_ammo_reload(
    time: Res<Time>,
    mut query: Query<(&mut AmmoReloadAnimation, &mut Node, Entity), With<AmmoBackground>>,
    mut black_bar_query: Query<&mut Node, (With<BlackBar>, Without<AmmoBackground>)>,
    mut commands: Commands,
    input_query: Query<&PlayerActionState>,
    weapon_query: Query<&Weapon, With<Player>>,
    weapon_state_query: Query<&WeaponState>,
) {
    for (mut anim, mut node, entity) in query.iter_mut() {
        anim.timer.tick(time.delta());
        let progress =
            (anim.timer.elapsed_secs() / anim.timer.duration().as_secs_f32()).clamp(0.0, 1.0);
        let percent = anim.from + (anim.to - anim.from) * progress;
        node.width = Val::Percent(percent * 100.0);

        // Update black bar accordingly
        if let Ok(mut black_bar) = black_bar_query.single_mut() {
            black_bar.width = Val::Percent((1.0 - percent) * 100.0);
        }

        let Ok(input) = input_query.single() else {
            return;
        };

        // Determine current selected slot
        let Ok(weapon) = weapon_query.single() else {
            return;
        };

        if weapon.0 != anim.original_weapon_entity {
            // L’arme a changé, on stoppe l’animation
            commands.entity(entity).remove::<AmmoReloadAnimation>();
            continue;
        }

        // Check ammo remaining
        let mut ammo_nonzero = false;
        if let Ok(weapon) = weapon_query.single()
            && let Ok(state) = weapon_state_query.get(weapon.0)
        {
            ammo_nonzero = state.current_ammo != 0;
        }

        let shoot_pressed = input.just_pressed(&PlayerAction::Shoot);

        if shoot_pressed && ammo_nonzero {
            commands.entity(entity).remove::<AmmoReloadAnimation>();
        }

        if anim.timer.finished() {
            node.width = Val::Percent(anim.to * 100.0);
            if let Ok(mut black_bar) = black_bar_query.single_mut() {
                black_bar.width = Val::Percent((1.0 - anim.to) * 100.0);
            }
            // Supprimer le composant à la fin de l'animation
            commands.entity(entity).remove::<AmmoReloadAnimation>();
        }
    }
}
