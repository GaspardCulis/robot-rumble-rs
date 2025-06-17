use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;

use crate::ui::UiAssets;

use super::Screen;

#[derive(Component)]
/// Marker for despawning
struct HomeMenu;

#[derive(Component)]
struct MenuEntry;

pub struct HomeMenuPlugin;
impl Plugin for HomeMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Home), spawn_menu)
            .add_systems(
                Update,
                update_background_size.run_if(in_state(Screen::Home)),
            );
    }
}

fn spawn_menu(mut commands: Commands, mut scene_builder: SceneBuilder, assets: Res<UiAssets>) {
    info!("Loading Home menu UI");
    let background_image = assets.background_image.clone();
    commands.ui_root().spawn_scene(
        ("ui/main.cob", "home"),
        &mut scene_builder,
        move |scene_handle| {
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
            scene_handle
                .get("multiplayer")
                .on_pressed(|mut next: ResMut<NextState<Screen>>| next.set(Screen::MatchMaking));
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
