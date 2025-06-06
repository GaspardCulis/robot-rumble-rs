use std::path::PathBuf;

use bevy::prelude::{Resource, States};
use clap::Parser;

pub mod assets;
pub mod core;
pub mod entities;
pub mod level;
pub mod misc;
pub mod network;
pub mod utils;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
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
    /// Number of players to match against
    #[arg(short, long, default_value_t = 2)]
    pub players: usize,
    /// File path for the level save file to be used instead of default worldgen
    #[arg(short, long)]
    pub level_path: Option<PathBuf>,
}
