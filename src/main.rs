#![warn(clippy::unwrap_used, clippy::panic)]
#![allow(clippy::needless_update, clippy::unnecessary_struct_initialization)]
#![feature(drain_filter, file_create_new)]

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
        cli::args::Command::PlayRandom => {
            Terminal::new(Root::with_board(Board::random()?))?.run()?
        }
    }

    Ok(())
}
