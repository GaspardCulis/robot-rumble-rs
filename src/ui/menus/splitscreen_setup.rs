use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;

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

fn spawn_menu(mut commands: Commands, mut scene_builder: SceneBuilder) {
    info!("Loading Splitscreen Setup menu UI");

    commands.ui_root().spawn_scene(
        ("ui/menu/splitscreen_setup.cob", "splitscreen_setup"),
        &mut scene_builder,
        move |scene_handle| {
            // Add marker struct
            scene_handle.insert(SplitscreenSetupMenu);
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
