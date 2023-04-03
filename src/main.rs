#![warn(clippy::unwrap_used, clippy::panic)]
#![allow(clippy::needless_update, clippy::unnecessary_struct_initialization)]
#![feature(drain_filter, file_create_new)]

use ai::stockfish::{self, eval};
use clap::Parser;
use cli::args::Cli;

extern crate anyhow;
extern crate clap;
extern crate intuitive;
extern crate rand;

pub mod ai;
pub mod cli;
pub mod core;
pub mod ui;
pub mod utils;

use anyhow::Result;
use intuitive::terminal::Terminal;

use crate::{core::board::Board, ui::root::Root};

fn main() -> Result<()> {
    let args = Cli::parse();

    match &args.command {
        cli::args::Command::Play => Terminal::new(Root::new())?.run()?,
        cli::args::Command::Eval(args) => {
            let fen = &args.fen;

            println!("{}", stockfish::eval(fen.clone())?);
        }
        cli::args::Command::EvalAfterMove(_) => todo!(),
        cli::args::Command::GenRandom => {
            let mut output = String::new();

            let random_board = Board::random()?;

            output.push_str(&random_board.fen());
            output.push(',');
            output.push_str(
                &random_board
                    .dataset()
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(""),
            );
            output.push(',');
            output.push_str(&eval(random_board.fen())?.to_string());

            println!("{output}");
        }
    }

    Ok(())
}
