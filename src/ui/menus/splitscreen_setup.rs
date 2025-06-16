use bevy::prelude::*;

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
        app.add_systems(OnEnter(Screen::Home), spawn_menu);
    }
}

fn spawn_menu(mut commands: Commands) {
    info!("Loading Splitscreen setup menu UI");
    commands.init_resource::<UiState>();
    commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            SplitscreenSetupMenu,
        ))
        .with_children(|spawner| {
            spawner.spawn((
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(60.0),
                    padding: UiRect::all(Val::Percent(3.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Text::new("+"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
            ));
            spawner.spawn((
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(60.0),
                    padding: UiRect::all(Val::Percent(3.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Text::new("-"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
            ));
        });
}
