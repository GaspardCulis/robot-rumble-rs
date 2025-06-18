use bevy::{input::gamepad::GamepadEvent, prelude::*};
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;

use super::Screen;

#[derive(Component)]
/// Marker for despawning
struct SplitscreenSetupMenu;

#[derive(ReactComponent)]
struct MatchInfo {
    player_count: usize,
}

pub struct SplitscreenSetupPlugin;
impl Plugin for SplitscreenSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::SplitscreenSetup), spawn_menu)
            .add_systems(
                Update,
                update_player_count.run_if(in_state(Screen::SplitscreenSetup)),
            )
            .add_systems(OnExit(Screen::SplitscreenSetup), despawn_menu);
    }
}

fn spawn_menu(mut commands: Commands, mut scene_builder: SceneBuilder, gamepads: Query<&Gamepad>) {
    info!("Loading Splitscreen Setup menu UI");

    commands.ui_root().spawn_scene(
        ("ui/menu/splitscreen_setup.cob", "splitscreen_setup"),
        &mut scene_builder,
        move |scene_handle| {
            // Add marker struct
            scene_handle.insert(SplitscreenSetupMenu);

            // Add reactive components
            scene_handle.insert_reactive(MatchInfo {
                player_count: gamepads.iter().count(),
            });
            let scene_id = scene_handle.id();

            // Spawn player config UIs
            scene_handle.get("container").update_on(
                entity_mutation::<MatchInfo>(scene_id),
                move |id: TargetId,
                      mut commands: Commands,
                      mut scene_builder: SceneBuilder,
                      info: Reactive<MatchInfo>| {
                    commands.entity(*id).despawn_related::<Children>();
                    let mut ui_builder = commands.ui_builder(*id);

                    for i in 1..=info.get(scene_id)?.player_count {
                        ui_builder.spawn_scene(
                            ("ui/menu/splitscreen_setup.cob", "player_info"),
                            &mut scene_builder,
                            |scene_handle| {
                                // Update text
                                scene_handle.get("text").update_text(format!("Player {i}"));
                            },
                        );
                    }

                    OK
                },
            );
        },
    );
}

fn update_player_count(
    mut commands: Commands,
    mut gamepad_events: EventReader<GamepadEvent>,
    mut info: ReactiveMut<MatchInfo>,
    gamepads: Query<&Gamepad>,
) {
    for event in gamepad_events.read() {
        if let GamepadEvent::Connection(_) = event {
            info.single_mut(&mut commands).1.player_count = gamepads.iter().count();
        }
    }
}

fn despawn_menu(
    mut commands: Commands,
    query: Query<Entity, With<SplitscreenSetupMenu>>,
) -> Result {
    let menu = query.single()?;
    commands.entity(menu).despawn();
    Ok(())
}
