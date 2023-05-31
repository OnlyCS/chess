#![warn(clippy::unwrap_used)]
#![allow(clippy::needless_update, clippy::unnecessary_struct_initialization)]
#![feature(drain_filter, file_create_new)]

extern crate anyhow;
extern crate clap;
extern crate intuitive;
extern crate rand;

pub mod cli;
pub mod core;
pub mod ui;
pub mod utils;

use anyhow::Result;
use clap::Parser;
use intuitive::terminal::Terminal;

use crate::{cli::args::Cli, core::hexapawn::HexapawnBoard, ui::hexapawn::Root};

fn main() -> Result<()> {
    let args = Cli::parse();

    Terminal::new(Root::with_board(HexapawnBoard::new(args.serial)))?.run()?;

    Ok(())
}
