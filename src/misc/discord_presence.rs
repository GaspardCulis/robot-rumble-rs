use bevy::prelude::*;
use discord_presence::Client;

const APPLICATION_ID: u64 = 1379103471302737931;

#[derive(Resource)]
struct DiscordPresence(Client);

pub struct DiscordPresencePlugin;
impl Plugin for DiscordPresencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(OnEnter(crate::GameState::InGame), set_ingame_presence);
    }
}

fn setup(mut commands: Commands) {
    let mut drpc = Client::new(APPLICATION_ID);
    drpc.on_ready(|_ctx| {
        info!("Discord presence ready");
    })
    .persist();

    drpc.start();

    commands.insert_resource(DiscordPresence(drpc));
}

fn set_ingame_presence(mut presence: ResMut<DiscordPresence>, args: Res<crate::Args>) {
    let _ = presence
        .0
        .set_activity(|activity| {
            activity
                .state(format!("Playing in {} player(s) match", args.players))
                .details("Competitive")
                .append_buttons(|button| {
                    button
                        .label("See code!")
                        .url("https://github.com/GaspardCulis/robot-rumble-rs")
                })
        })
        .inspect_err(|e| error!("Failed to set discord activity: {}", e));
}
