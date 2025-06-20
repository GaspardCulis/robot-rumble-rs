use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;

use crate::{
    GameState,
    entities::player::{
        Weapon,
        inventory::Arsenal,
        weapon::{
            WeaponState,
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

#[derive(Component, Reflect)]
/// Points to the `Player` entity owning the HUD
struct PlayerHud(Entity);

#[derive(ReactComponent, Default, PartialEq)]
struct CurrentWeaponInfo {
    current_ammo: usize,
    magazine_size: usize,
}

#[derive(ReactComponent, Default, PartialEq)]
struct CurrentWeaponSprite(Handle<Image>);

pub struct HUDPlugin;
impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_menu).run_if(in_state(GameState::InGame)))
            .add_systems(
                FixedUpdate,
                (update_weapon_info, update_weapon_sprite).run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), despawn_menu);
    }
}

fn spawn_menu(
    mut commands: Commands,
    mut scene_builder: SceneBuilder,
    arsenals: Query<(Entity, &Arsenal), Changed<Arsenal>>,
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

            for (player, arsenal) in arsenals.iter() {
                scene_handle.spawn_scene(("ui/hud.cob", "hud"), move |scene_handle| {
                    let scene_id = scene_handle.id();

                    scene_handle.insert(PlayerHud(player));
                    scene_handle.insert_reactive(CurrentWeaponInfo::default());
                    scene_handle.insert_reactive(CurrentWeaponSprite::default());
                    scene_handle.get("vbox::hbox::bullets_count").update_on(
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
                    for (weapon_type, _) in arsenal.0.iter() {
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
                                scene_handle.update_text(name);
                            },
                        );
                    }
                });
            }
        },
    );
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
        let (weapon_state, weapon_stats) = weapon_query.get(weapon.0)?;

        let _ = info.set_if_neq(
            &mut commands,
            entity,
            CurrentWeaponInfo {
                current_ammo: weapon_state.current_ammo,
                magazine_size: weapon_stats.magazine_size,
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

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<HudRoot>>) -> Result {
    let menu = query.single()?;
    commands.entity(menu).despawn();
    Ok(())
}
