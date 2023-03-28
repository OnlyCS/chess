#![warn(clippy::unwrap_used, clippy::panic)]
#![allow(clippy::needless_update)]
#![feature(drain_filter)]

extern crate anyhow;
extern crate intuitive;
extern crate rand;
extern crate rustneat;

pub mod ai;
pub mod core;
pub mod ui;
pub mod utils;

use anyhow::Result;

use crate::{ai::stockfish, core::board::Board};

fn main() -> Result<()> {
    // let mut t = Terminal::new(Root::new())?;
    // t.run()?;

    // Ok(())

    for _ in 0..100 {
        let fen = Board::random()?.fen();
        let score = stockfish::eval(fen.clone())?;
        println!("{}: score {}", fen, score);
    }

    Ok(())
}
