use bevy::prelude::*;
use lightyear::prelude::*;
use network::{net_config, shared_config};

mod core;
mod entities;
mod network;
mod utils;

fn main() {
    let server_config = server::ServerConfig {
        shared: shared_config(Mode::Separate),
        net: vec![net_config()],
        ..Default::default()
    };

    let server_plugin = server::ServerPlugins::new(server_config);

    let mut app = App::new();

    app.add_plugins(server_plugin).run();
}
