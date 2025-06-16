use bevy::prelude::*;

use crate::ui::widgets::UIButton;

use super::Screen;

#[derive(Resource, Default)]
struct UiState {
    num_players: usize,
}

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

fn spawn_menu(mut commands: Commands) {
    info!("Loading Splitscreen setup menu UI");
    commands.insert_resource(UiState { num_players: 2 });

    commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            SplitscreenSetupMenu,
        ))
        .with_children(|spawner| {
            spawner
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|spawner| {
                    spawner.spawn(UIButton::default().with_text("+")).observe(
                        |_: Trigger<Pointer<Click>>, mut state: ResMut<UiState>| {
                            state.num_players += 1;
                        },
                    );
                    spawner.spawn(UIButton::default().with_text("-")).observe(
                        |_: Trigger<Pointer<Click>>, mut state: ResMut<UiState>| {
                            state.num_players = state.num_players.checked_sub(1).unwrap_or(0);
                        },
                    );
                });
        });
}

fn despawn_menu(
    mut commands: Commands,
    query: Query<Entity, With<SplitscreenSetupMenu>>,
) -> Result {
    let menu = query.single()?;
    commands.entity(menu).despawn();
    Ok(())
}
