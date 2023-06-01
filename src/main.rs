#![warn(clippy::unwrap_used)]
#![allow(clippy::needless_update, clippy::unnecessary_struct_initialization)]
#![feature(drain_filter, file_create_new, let_chains)]

extern crate anyhow;
extern crate clap;
extern crate intuitive;
extern crate rand;

pub mod cli;
pub mod core;
pub mod grbl;
pub mod ui;
pub mod utils;

use anyhow::*;
use clap::Parser;
use intuitive::terminal::Terminal;

use crate::{
    cli::args::Cli, core::hexapawn::HexapawnBoard, ui::hex_ui::Root as HexRoot,
    ui::root::Root as ChessRoot,
};

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.hexapawn {
        Terminal::new(HexRoot::with_board(HexapawnBoard::new(args.serial)))?.run()?;
    } else if args.serial {
        bail!("serial is only supported for hexapawn. Try using \"-x\"")
    } else {
        Terminal::new(ChessRoot::new())?.run()?;
    }

    Ok(())
}
