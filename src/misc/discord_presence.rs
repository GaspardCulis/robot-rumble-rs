use bevy::prelude::*;
use discord_presence::{Client, Event};

const APPLICATION_ID: u64 = 1379103471302737931;

#[derive(Resource)]
struct DiscordPresence(Client);

pub struct DiscordPresencePlugin;
impl Plugin for DiscordPresencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, args: Res<crate::Args>) {
    let mut drpc = Client::new(APPLICATION_ID);
    drpc.on_ready(|_ctx| {
        info!("Discord presence ready");
    })
    .persist();

    drpc.start();

    let _ = drpc
        .set_activity(|activity| {
            activity.state(format!("Playing in {} player(s) match", args.players))
        })
        .inspect_err(|e| error!("Failed to set discord activity: {}", e));

    commands.insert_resource(DiscordPresence(drpc));
}
