use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;

use crate::{Args, GameMode, GameState, ui::UIAssets};

use super::Screen;

#[derive(Component)]
/// Marker for despawning
struct HomeMenu;

pub struct HomeMenuPlugin;
impl Plugin for HomeMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Screen::Home),
            (check_cmdline_args, spawn_menu).chain(),
        )
        .add_systems(
            Update,
            update_background_size.run_if(in_state(Screen::Home)),
        )
        .add_systems(OnExit(Screen::Home), despawn_menu);
    }
}

fn check_cmdline_args(
    args: Res<Args>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_gamestate: ResMut<NextState<GameState>>,
) {
    // Don't require going through menus if args are explicitly given
    if args.mode == GameMode::LocalPlay {
        next_screen.set(Screen::SplitscreenSetup);
    } else
    // TODO: Less cringe way of checking if players arg is given
    if args.mode == GameMode::Synctest || args.players != 2 {
        next_screen.set(Screen::None);
        next_gamestate.set(GameState::MatchMaking);
    }
}

fn spawn_menu(mut commands: Commands, mut scene_builder: SceneBuilder, assets: Res<UIAssets>) {
    info!("Loading Home menu UI");

    let background_image = assets.background_image.clone();

    commands.ui_root().spawn_scene(
        ("ui/menu/home.cob", "home"),
        &mut scene_builder,
        move |scene_handle| {
            // Add marker struct
            scene_handle.insert(HomeMenu);

            // Set background image
            scene_handle.get("background").modify(
                move |mut entity_commands: EntityCommands<'_>| {
                    entity_commands.insert(
                        ImageNode::new(background_image.clone())
                            .with_rect(Rect::new(0.0, 0.0, 2560.0, 1440.0)),
                    );
                },
            );

            // Add click observers
            scene_handle.get("multiplayer").on_pressed(
                |mut next_screen: ResMut<NextState<Screen>>| {
                    next_screen.set(Screen::MatchmakingSetup);
                },
            );
            scene_handle
                .get("local")
                .on_pressed(|mut next: ResMut<NextState<Screen>>| {
                    next.set(Screen::SplitscreenSetup)
                });
            scene_handle
                .get("settings")
                .on_pressed(|mut next: ResMut<NextState<Screen>>| next.set(Screen::Home)); // TODO: Implement menu
            scene_handle
                .get("credits")
                .on_pressed(|mut next: ResMut<NextState<Screen>>| next.set(Screen::Home)); // TODO: Implement menu
            scene_handle
                .get("quit")
                .on_pressed(|mut exit: EventWriter<AppExit>| {
                    exit.write(AppExit::Success);
                });
        },
    );
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<HomeMenu>>) -> Result {
    let menu = query.single()?;
    commands.entity(menu).despawn();
    Ok(())
}

fn update_background_size(
    mut images: Query<(&mut Node, &ImageNode)>,
    window: Query<&Window>,
) -> Result {
    let window = window.single()?;
    let window_ar = window.size().x / window.size().y;

    for (mut node, image_node) in images.iter_mut() {
        if let Some(rect) = image_node.rect {
            let image_node_ar = rect.size().x / rect.size().y;
            if image_node_ar > window_ar {
                node.min_width = default();
                node.min_height = Val::Percent(100.0);
            } else {
                node.min_width = Val::Percent(100.0);
                node.min_height = default();
            }
        }
    }

    Ok(())
}
