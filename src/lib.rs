use std::path::PathBuf;

use bevy::prelude::{Resource, States};
use clap::Parser;

pub mod assets;
pub mod core;
pub mod entities;
pub mod level;
pub mod misc;
pub mod network;
pub mod ui;
pub mod utils;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    None,
    MatchMaking,
    WorldGen,
    InGame,
}

#[derive(clap::ValueEnum, serde::Serialize, Eq, PartialEq, Clone, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub enum GameMode {
    #[default]
    /// Runs the game in peer-to-peer mode
    Multiplayer,
    /// Runs the game in localplay mode (splitscreen)
    LocalPlay,
    /// Runs the game in synctest mode
    Synctest,
}

// TODO: Should be defined by `robot-rumble` main
#[derive(Parser, Resource, Debug)]
pub struct Args {
    /// Game mode
    #[arg(value_enum, short, long, default_value_t)]
    pub mode: GameMode,
    /// Number of players to match against
    #[arg(short, long, default_value_t = 2)]
    pub players: usize,
    /// File path for the level save file to be used instead of default worldgen
    #[arg(short, long)]
    pub level_path: Option<PathBuf>,
}
