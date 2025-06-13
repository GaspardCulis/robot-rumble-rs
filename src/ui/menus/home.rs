use bevy::prelude::*;

use crate::ui::UiAssets;

use super::Screen;

#[derive(Resource, Default)]
struct UiState {
    selected_menu_entry: Option<Entity>,
}

#[derive(Component)]
/// Marker for despawning
struct HomeMenu;

#[derive(Component)]
struct MenuEntry;

#[derive(Component)]
struct NextStateOnClick(Screen);

pub struct HomeMenuPlugin;
impl Plugin for HomeMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Home), spawn_menu)
            .add_systems(
                Update,
                (update_menu_entry_scale, update_background_size).run_if(in_state(Screen::Home)),
            )
            .add_systems(OnExit(Screen::Home), despawn_menu);
    }
}

fn spawn_menu(mut commands: Commands, assets: Res<UiAssets>) {
    info!("Loading Home menu UI");
    commands.init_resource::<UiState>();
    commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Percent(3.0)),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            HomeMenu,
        ))
        .with_children(|spawner| {
            spawner
                .spawn(c_menu_entry("Play", Screen::MatchMaking))
                .observe(click_observer)
                .observe(hover_in_observer)
                .observe(hover_out_observer);
            spawner
                .spawn(c_menu_entry("Settings", Screen::Home)) // Set to correct state when implemented
                .observe(click_observer)
                .observe(hover_in_observer)
                .observe(hover_out_observer);

            spawner
                .spawn(c_menu_entry("Credits", Screen::Home)) // Same
                .observe(click_observer)
                .observe(hover_in_observer)
                .observe(hover_out_observer);

            spawner
                .spawn(c_menu_entry("Quit", Screen::Home))
                .observe(hover_in_observer)
                .observe(hover_out_observer)
                .observe(
                    |_: Trigger<Pointer<Click>>, mut exit: EventWriter<AppExit>| {
                        // Close the app on click
                        exit.write(AppExit::Success);
                    },
                );

            spawner.spawn((
                ImageNode::new(assets.background_image.clone())
                    .with_rect(Rect::new(0.0, 0.0, 2560.0, 1440.0)),
                Node {
                    min_width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    ..default()
                },
                ZIndex(-1),
            ));
        });
}

fn update_menu_entry_scale(
    mut query: Query<(Entity, &mut Transform), With<MenuEntry>>,
    ui_state: Res<UiState>,
    time: Res<Time>,
) {
    for (entity, mut transform) in query.iter_mut() {
        let selected = ui_state
            .selected_menu_entry
            .is_some_and(|selected| selected == entity);

        let target_scale = if selected {
            Vec3::splat(1.2)
        } else {
            Vec3::ONE
        };

        transform.scale = transform.scale.lerp(target_scale, time.delta_secs() * 16.0);
    }
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

fn c_menu_entry(text: impl Into<String>, next_state_on_click: Screen) -> impl Bundle {
    (
        MenuEntry,
        Node {
            margin: UiRect::bottom(Val::Px(8.0)),
            ..default()
        },
        Text::new(text),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
        NextStateOnClick(next_state_on_click),
    )
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<HomeMenu>>) -> Result {
    let menu = query.single()?;
    commands.entity(menu).despawn();
    Ok(())
}

fn hover_in_observer(trigger: Trigger<Pointer<Over>>, mut ui_state: ResMut<UiState>) {
    let event = trigger.event();
    ui_state.selected_menu_entry = Some(event.target);
}

fn hover_out_observer(trigger: Trigger<Pointer<Out>>, mut ui_state: ResMut<UiState>) {
    let event = trigger.event();
    if ui_state
        .selected_menu_entry
        .is_some_and(|selected| selected == event.target)
    {
        ui_state.selected_menu_entry = None;
    }
}

fn click_observer(
    mut trigger: Trigger<Pointer<Click>>,
    mut next_menus: ResMut<NextState<Screen>>,
    query: Query<&NextStateOnClick>,
) {
    let event = trigger.event();

    if event.button == PointerButton::Primary
        && let Ok(next_state) = query.get(event.target)
    {
        next_menus.set(next_state.0.clone());
    };

    trigger.propagate(false);
}
