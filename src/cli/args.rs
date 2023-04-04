use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author = "OnlyCS", version, about = "Chess GUI and AI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Play,
    PlayRandom,
}
