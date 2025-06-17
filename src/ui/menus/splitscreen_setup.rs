use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;

use crate::ui::UiAssets;

use super::Screen;

#[derive(Component)]
/// Marker for despawning
struct SplitscreenSetupMenu;

pub struct SplitscreenSetupPlugin;
impl Plugin for SplitscreenSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::SplitscreenSetup), spawn_menu)
            .add_systems(OnExit(Screen::SplitscreenSetup), despawn_menu);
    }
}

fn spawn_menu(mut commands: Commands, mut scene_builder: SceneBuilder, assets: Res<UiAssets>) {
    info!("Loading Splitscreen Setup menu UI");

    let gamepad_icon = assets.gamepad_icon.clone();

    commands.ui_root().spawn_scene(
        ("ui/menu/splitscreen_setup.cob", "splitscreen_setup"),
        &mut scene_builder,
        move |scene_handle| {
            // Add marker struct
            scene_handle.insert(SplitscreenSetupMenu);

            for i in 1..=3 {
                let gamepad_icon = gamepad_icon.clone();

                scene_handle.get("container").spawn_scene(
                    ("ui/menu/splitscreen_setup.cob", "player_config"),
                    move |scene_handle| {
                        // Update text
                        scene_handle.get("text").update_text(format!("Player {i}"));
                        // Set image handles
                        scene_handle.get("gamepad_icon").modify(
                            move |mut entity_commands: EntityCommands<'_>| {
                                entity_commands.insert(ImageNode::new(gamepad_icon.clone()));
                            },
                        );
                    },
                );
            }
        },
    );
}

fn despawn_menu(
    mut commands: Commands,
    query: Query<Entity, With<SplitscreenSetupMenu>>,
) -> Result {
    let menu = query.single()?;
    commands.entity(menu).despawn();
    Ok(())
}
