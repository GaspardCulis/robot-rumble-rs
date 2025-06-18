use bevy::{input::gamepad::GamepadEvent, prelude::*};
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use rand::Rng;

use crate::{
    GameState,
    core::{camera, inputs},
    entities::player::Player,
    network,
};

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
            .add_systems(OnExit(Screen::SplitscreenSetup), despawn_menu)
            // Hack for starting the game
            .add_systems(
                OnEnter(GameState::WorldGen),
                |mut next: ResMut<NextState<GameState>>| next.set(GameState::InGame),
            );
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

            // Add button handlers
            scene_handle
                .get("start_button")
                .on_pressed(handle_start_button_press);

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

// FIX: Too much boilerplate to start a match; create a better framework.
fn handle_start_button_press(
    mut commands: Commands,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_gamestate: ResMut<NextState<GameState>>,
    mut args: ResMut<crate::Args>,
    gamepads: Query<Entity, With<Gamepad>>,
) {
    info!("Starting local play match");
    next_screen.set(Screen::None);
    args.localplay = true;

    // Load world
    let seed = rand::rng().random();
    commands.insert_resource(network::SessionSeed(seed));

    // Spawn players with associated gamepad input maps
    let player_bundles = gamepads
        .iter()
        .enumerate()
        .map(|(i, gamepad)| {
            let mut input_map = inputs::default_input_map();
            input_map.set_gamepad(gamepad);
            (Player { handle: i }, input_map)
        })
        .collect::<Vec<_>>();

    let mut session_builder = bevy_ggrs::ggrs::SessionBuilder::<network::SessionConfig>::new()
        .with_num_players(player_bundles.len())
        .with_input_delay(2);

    for (player, input_map) in player_bundles.into_iter() {
        session_builder = session_builder
            .add_player(bevy_ggrs::ggrs::PlayerType::Local, player.handle)
            .expect("Failed to add player");

        commands.spawn((player, input_map));
    }

    let mut socket = bevy_matchbox::MatchboxSocket::new_unreliable(
        "wss://matchbox.gasdev.fr/extreme_bevy?next=1",
    );
    let ggrs_session = session_builder
        .start_p2p_session(socket.take_channel(0).unwrap())
        .expect("Failed to start P2P session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));

    next_gamestate.set(GameState::WorldGen);
}

fn despawn_menu(
    mut commands: Commands,
    query: Query<Entity, With<SplitscreenSetupMenu>>,
) -> Result {
    let menu = query.single()?;
    commands.entity(menu).despawn();
    Ok(())
}
