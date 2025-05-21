use bevy::{color::palettes::css, prelude::*};

use crate::{GameState, entities::player::weapons};

const WEAPON_SLOTS: [weapons::WeaponType; 3] = [
    weapons::WeaponType::Pistol,
    weapons::WeaponType::Shotgun,
    weapons::WeaponType::Rifle,
];

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_arsenal_hud);
    }
}

fn spawn_arsenal_hud(
    mut commands: Commands,
    config_handle: Res<weapons::WeaponsConfigHandle>,
    config_assets: Res<Assets<weapons::config::WeaponsConfig>>,
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
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: (0..WEAPON_SLOTS.len())
                        .map(|_| GridTrack::auto())
                        .collect(),
                    grid_template_rows: vec![GridTrack::auto()],
                    ..default()
                })
                .with_children(|builder| {
                    for weapon_type in WEAPON_SLOTS {
                        let weapon_config = weapons_config
                            .0
                            .get(&weapon_type)
                            .expect("Failed to get weapon config");
                        let skin = weapon_config.skin.clone();

                        builder.spawn((
                            Node {
                                display: Display::Grid,
                                width: Val::Px(128.0),
                                height: Val::Px(128.0),
                                margin: UiRect::all(Val::Px(24.0)),
                                ..default()
                            },
                            ImageNode::new(asset_server.load(skin.sprite)),
                            Outline::new(Val::Px(4.0), Val::ZERO, css::CRIMSON.into()),
                        ));
                    }
                });
        });
}
