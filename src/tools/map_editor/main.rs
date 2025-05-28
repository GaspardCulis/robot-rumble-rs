use clap::Parser as _;
use robot_rumble::*;

fn main() {
    let args = Args::parse();
    println!("Hallo {}", args.players);
}
