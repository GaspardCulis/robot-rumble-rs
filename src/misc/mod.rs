use bevy::app::Plugin;

pub mod discord_presence;

pub struct MiscPlugins;
impl Plugin for MiscPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(discord_presence::DiscordPresencePlugin);
    }
}
