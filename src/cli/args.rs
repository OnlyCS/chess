use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author = "OnlyCS", version, about = "Chess GUI and AI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Play,
    Eval(EvalArgs),
    EvalAfterMove(EvalAfterMoveArgs),
    GenRandom,
}

#[derive(Args)]
pub struct EvalArgs {
    pub fen: String,
}

#[derive(Args)]
pub struct EvalAfterMoveArgs {
    pub fen: String,
    pub r#move: String,
}
