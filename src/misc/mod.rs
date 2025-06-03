use bevy::app::Plugin;

#[cfg(feature = "discord_presence")]
pub mod discord_presence;

pub struct MiscPlugins;
impl Plugin for MiscPlugins {
    #[allow(unused_variables)] // As currently there is only the DiscordPresencePlugin that can be disabled
    fn build(&self, app: &mut bevy::app::App) {
        #[cfg(feature = "discord_presence")]
        app.add_plugins(discord_presence::DiscordPresencePlugin);
    }
}
