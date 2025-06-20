use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;

use crate::{GameMode, GameState};

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

fn spawn_menu(mut commands: Commands, mut scene_builder: SceneBuilder, args: Res<crate::Args>) {
    info!("Loading Splitscreen Setup menu UI");

    commands.ui_root().spawn_scene(
        ("ui/menu/matchmaking_setup.cob", "matchmaking_setup"),
        &mut scene_builder,
        move |scene_handle| {
            // Add marker struct
            scene_handle.insert(MatchmakingSetupMenu);

            // Add button handlers
            scene_handle
                .get("start_button")
                .on_pressed(handle_start_button_press);
            scene_handle
                .get("back_button")
                .on_pressed(|mut next: ResMut<NextState<Screen>>| next.set(Screen::Home));

            // Spawn game mode buttons
            for n in 2..=5 {
                scene_handle.get("container").spawn_scene(
                    ("ui/menu/matchmaking_setup.cob", "gamemode"),
                    |scene_handle| {
                        // Update text
                        scene_handle.get("text").update_text(format!("{n}P Match"));

                        // Add select listener
                        scene_handle.on_select(move |mut args: ResMut<crate::Args>| {
                            args.players = n;
                        });

                        // Set the default selection
                        if n == args.players {
                            let entity = scene_handle.id();
                            scene_handle.react().entity_event(entity, Select);
                        }
                    },
                );
            }
        },
    );
}

fn handle_start_button_press(
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_gamestate: ResMut<NextState<GameState>>,
    mut args: ResMut<crate::Args>,
) {
    args.mode = GameMode::Multiplayer;

    next_screen.set(Screen::None);
    next_gamestate.set(GameState::MatchMaking);
}

fn despawn_menu(
    mut commands: Commands,
    query: Query<Entity, With<MatchmakingSetupMenu>>,
) -> Result {
    let menu = query.single()?;
    commands.entity(menu).despawn();
    Ok(())
}
