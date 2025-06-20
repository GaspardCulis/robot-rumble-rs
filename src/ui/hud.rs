use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;

use crate::{
    GameState,
    entities::player::{inventory::Arsenal, weapon::config::WeaponType},
};

#[derive(Component, Reflect)]
#[require(Name::new("HUD"))]
/// Marker for despawning.
/// Marks the root HUD container, which can contain HUDs for multiple players.
struct Hud;

pub struct HUDPlugin;
impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_menu.run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), despawn_menu);
    }
}

fn spawn_menu(
    mut commands: Commands,
    mut scene_builder: SceneBuilder,
    arsenals: Query<&Arsenal, Changed<Arsenal>>,
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
            scene_handle.insert(Hud);

            for arsenal in arsenals.iter() {
                scene_handle.spawn_scene(("ui/hud.cob", "hud"), move |scene_handle| {
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

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<Hud>>) -> Result {
    let menu = query.single()?;
    commands.entity(menu).despawn();
    Ok(())
}
