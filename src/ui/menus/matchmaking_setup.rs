use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;

use crate::GameState;

use super::Screen;

#[derive(Component)]
/// Marker for despawning
struct MatchmakingSetupMenu;

pub struct MatchmakingSetupPlugin;
impl Plugin for MatchmakingSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::MatchmakingSetup), spawn_menu)
            .add_systems(OnExit(Screen::MatchmakingSetup), despawn_menu);
    }
}

fn spawn_menu(mut commands: Commands, mut scene_builder: SceneBuilder) {
    info!("Loading Splitscreen Setup menu UI");

    commands.ui_root().spawn_scene(
        ("ui/menu/matchmaking_setup.cob", "matchmaking_setup"),
        &mut scene_builder,
        move |scene_handle| {
            // Add marker struct
            scene_handle.insert(MatchmakingSetupMenu);

            // Spawn game mode buttons
            for n in 2..=5 {
                scene_handle.get("container").spawn_scene(
                    ("ui/menu/matchmaking_setup.cob", "gamemode"),
                    |scene_handle| {
                        // Update text
                        scene_handle.get("text").update_text(format!("{n}P Match"));

                        // Set 2 player match as selected by default
                        if n == 2 {
                            let entity = scene_handle.id();
                            scene_handle.react().entity_event(entity, Select);
                        }
                    },
                );
            }
        },
    );
}

fn despawn_menu(
    mut commands: Commands,
    query: Query<Entity, With<MatchmakingSetupMenu>>,
) -> Result {
    let menu = query.single()?;
    commands.entity(menu).despawn();
    Ok(())
}
