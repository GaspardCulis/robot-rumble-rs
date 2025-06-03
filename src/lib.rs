use bevy::prelude::{Resource, States};
use clap::Parser;

pub mod core;
pub mod entities;
pub mod level;
pub mod misc;
pub mod network;
pub mod utils;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    MatchMaking,
    WorldGen,
    InGame,
}

// TODO: Should be defined by `robot-rumble` main
#[derive(Parser, Resource, Debug)]
pub struct Args {
    /// Runs the game in synctest mode
    #[clap(long)]
    pub synctest: bool,
    #[arg(short, long, default_value_t = 2)]
    pub players: usize,
}
