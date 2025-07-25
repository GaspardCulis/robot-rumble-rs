use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_ggrs::LocalPlayers;

use crate::{
    GameState,
    entities::player::{
        Percentage, Player, Weapon,
        inventory::Arsenal,
        weapon::{
            Owner, WeaponState,
            assets::WeaponsAssets,
            config::{WeaponStats, WeaponType},
        },
    },
};

#[derive(Component, Reflect)]
#[require(Name::new("HUD"))]
/// Marker for despawning.
/// Marks the root HUD container, which can contain HUDs for multiple players.
struct HudRoot;

#[derive(Component)]
/// Marker component for `weapon_entry` scene holding a reference to a weapon entity.
/// Used for updating selected's weapon style.
struct WeaponEntry(Entity);

#[derive(Component, Reflect)]
/// Points to the `Player` entity owning the HUD
struct PlayerHud(Entity);

#[derive(ReactComponent, Default, PartialEq)]
struct PlayerInfo {
    percentage: f32,
}

#[derive(ReactComponent, Default, PartialEq)]
struct CurrentWeaponInfo {
    current_ammo: usize,
    magazine_size: usize,
    reload_time: f32,
    remaining_reload_time: f32,
}

#[derive(ReactComponent, Default, PartialEq)]
struct CurrentWeaponSprite(Handle<Image>);

pub struct HUDPlugin;
impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            // Run in `FixedUpdate` because UI info updates do not need to be blazingly fast
            FixedUpdate,
            (
                spawn_hud,
                update_player_info,
                update_weapon_info,
                update_weapon_sprite,
                update_weapon_entry_style,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnExit(GameState::InGame), despawn_menu);
    }
}

fn spawn_hud(
    mut commands: Commands,
    mut scene_builder: SceneBuilder,
    arsenals: Query<(Entity, &Player, &Arsenal), Changed<Arsenal>>,
    local_players: Res<LocalPlayers>,
) {
    if arsenals.is_empty() {
        return;
    }

    info!("Updating Head-Up Display UI");

    commands.ui_root().spawn_scene(
        ("ui/hud.cob", "hud_container"),
        &mut scene_builder,
        move |scene_handle| {
            // Add marker struct
            scene_handle.insert(HudRoot);

            for (player_entity, player, arsenal) in arsenals.iter() {
                if !local_players.0.contains(&player.handle) {
                    // Don't spawn HUD for local players
                    continue;
                }

                scene_handle.spawn_scene(("ui/hud.cob", "hud"), move |scene_handle| {
                    let scene_id = scene_handle.id();

                    scene_handle.insert(PlayerHud(player_entity));
                    scene_handle.insert_reactive(PlayerInfo::default());
                    scene_handle.insert_reactive(CurrentWeaponInfo::default());
                    scene_handle.insert_reactive(CurrentWeaponSprite::default());
                    scene_handle
                        .get("vbox::hbox::infos::percentage::text")
                        .update_on(
                            entity_mutation::<PlayerInfo>(scene_id),
                            move |id: TargetId, info: Reactive<PlayerInfo>, mut e: TextEditor| {
                                let player_info = info.get(scene_id)?;
                                write_text!(e, id.0, "{:.1}%", player_info.percentage * 100.0);
                                OK
                            },
                        );

                    scene_handle
                        .get("vbox::hbox::infos::bullets_count::text")
                        .update_on(
                            entity_mutation::<CurrentWeaponInfo>(scene_id),
                            move |id: TargetId,
                                  info: Reactive<CurrentWeaponInfo>,
                                  mut e: TextEditor| {
                                let weapon_info = info.get(scene_id)?;
                                write_text!(
                                    e,
                                    id.0,
                                    "{}/{}",
                                    weapon_info.current_ammo,
                                    weapon_info.magazine_size
                                );
                                OK
                            },
                        );

                    scene_handle.get("vbox::hbox::reload_bg").update_on(
                        entity_mutation::<CurrentWeaponInfo>(scene_id),
                        move |id: TargetId,
                              mut query: Query<&mut Node>,
                              info: Reactive<CurrentWeaponInfo>| {
                            let info = info.get(scene_id)?;
                            let mut node = query.get_mut(id.0)?;
                            node.width = Val::Percent(
                                100.0 * (1.0 - info.remaining_reload_time / info.reload_time),
                            );
                            OK
                        },
                    );

                    scene_handle
                        .get("vbox::hbox::a_weapon_preview::image")
                        .update_on(
                            entity_mutation::<CurrentWeaponSprite>(scene_id),
                            move |id: TargetId,
                                  mut query: Query<&mut ImageNode>,
                                  info: Reactive<CurrentWeaponSprite>| {
                                let weapon_sprite = info.get(scene_id)?;
                                let mut image = query.get_mut(id.0)?;
                                image.image = weapon_sprite.0.clone();
                                OK
                            },
                        );

                    let mut weapons_list = scene_handle.get("vbox::weapons_list");
                    for (weapon_type, weapon_entity) in arsenal.0.iter() {
                        let name = match weapon_type {
                            WeaponType::Pistol => "Pistol",
                            WeaponType::Rifle => "Rifle",
                            WeaponType::Shotgun => "Shotgun",
                            WeaponType::Sniper => "Sniper",
                            WeaponType::BlackholeGun => "BHL",
                            _ => unimplemented!(),
                        };

                        weapons_list.spawn_scene(
                            ("ui/hud.cob", "weapon_entry"),
                            move |scene_handle| {
                                scene_handle.insert(WeaponEntry(*weapon_entity));
                                scene_handle.get("text").update_text(name);
                            },
                        );
                    }
                });
            }
        },
    );
}

fn update_player_info(
    mut commands: Commands,
    mut info: ReactiveMut<PlayerInfo>,
    huds: Query<(Entity, &PlayerHud)>,
    player_query: Query<&Percentage>,
) -> Result {
    for (entity, hud) in huds.iter() {
        let percentage = player_query.get(hud.0)?;

        let _ = info.set_if_neq(
            &mut commands,
            entity,
            PlayerInfo {
                percentage: percentage.0,
            },
        );
    }

    Ok(())
}

fn update_weapon_info(
    mut commands: Commands,
    mut info: ReactiveMut<CurrentWeaponInfo>,
    huds: Query<(Entity, &PlayerHud)>,
    player_query: Query<&Weapon>,
    weapon_query: Query<(&WeaponState, &WeaponStats)>,
) -> Result {
    for (entity, hud) in huds.iter() {
        let weapon = player_query.get(hud.0)?;
        let Ok((weapon_state, weapon_stats)) = weapon_query.get(weapon.0) else {
            debug!("Weapon state and stats still not loaded");
            continue;
        };

        let _ = info.set_if_neq(
            &mut commands,
            entity,
            CurrentWeaponInfo {
                current_ammo: weapon_state.current_ammo,
                magazine_size: weapon_stats.magazine_size,
                reload_time: weapon_stats.reload_time.as_secs_f32(),
                remaining_reload_time: weapon_state.reload_timer.remaining().as_secs_f32(),
            },
        );
    }

    Ok(())
}

fn update_weapon_sprite(
    mut commands: Commands,
    mut sprites: ReactiveMut<CurrentWeaponSprite>,
    huds: Query<(Entity, &PlayerHud)>,
    player_query: Query<&Weapon>,
    weapon_query: Query<&WeaponType>,
    weapon_assets: Res<WeaponsAssets>,
) -> Result {
    for (entity, hud) in huds.iter() {
        // FIX: Updated each frame, not ideal
        let weapon = player_query.get(hud.0)?;
        let weapon_type = weapon_query.get(weapon.0)?;
        let weapon_assets = weapon_assets
            .get(weapon_type)
            .ok_or(BevyError::from("Failed to get WeaponAssets"))?;

        let _ = sprites.set_if_neq(
            &mut commands,
            entity,
            CurrentWeaponSprite(weapon_assets.skin.clone()),
        );
    }

    Ok(())
}

fn update_weapon_entry_style(
    mut commands: Commands,
    entries: Query<(Entity, &WeaponEntry, Has<BackgroundColor>)>,
    weapon_query: Query<Has<Owner>>,
) {
    for (node_entity, weapon_entry, is_entry_highlighted) in entries.iter() {
        let is_weapon_selected = weapon_query.get(weapon_entry.0).unwrap_or_else(|e| {
            warn!("Invalid query result: {}", e);
            false
        });

        if is_weapon_selected && !is_entry_highlighted {
            commands
                .entity(node_entity)
                .insert(BackgroundColor(Srgba::hex("#353535a0").unwrap().into()));
        } else if !is_weapon_selected && is_entry_highlighted {
            commands.entity(node_entity).remove::<BackgroundColor>();
        }
    }
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<HudRoot>>) -> Result {
    let menu = query.single()?;
    commands.entity(menu).despawn();
    Ok(())
}
