use bevy::color::palettes::css::{BLACK, BLUE, WHITE};
use bevy::prelude::*;

use crate::GameState;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.20, 0.20, 0.20);
const PRESSED_BUTTON: Color = Color::srgb(0.10, 0.10, 0.10);

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_button)
            .add_systems(Update, button_system);
    }
}

fn spawn_button(mut commands: Commands) {
    let container_node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };

    let button_node = Node {
        width: Val::Px(150.0),
        height: Val::Px(65.0),
        border: UiRect::all(Val::Px(5.0)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_node = Text::new("Button");
    let button_text_color = TextColor(Color::srgb(0.9, 0.9, 0.9));
    let button_text_font = TextFont {
        font_size: 40.0,
        ..default()
    };

    let container = commands.spawn(container_node).id();
    let button = commands
        .spawn((
            button_node,
            Button,
            BorderColor(Color::BLACK),
            BackgroundColor(NORMAL_BUTTON),
        ))
        .id();

    let button_text = commands
        .spawn((button_text_node, button_text_color, button_text_font))
        .id();
    commands.entity(button).add_children(&[button_text]);
    commands.entity(container).add_children(&[button]);
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.0 = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = BLUE.into();
            }
            Interaction::Hovered => {
                text.0 = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = WHITE.into();
            }
            Interaction::None => {
                text.0 = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = BLACK.into();
            }
        }
    }
}
