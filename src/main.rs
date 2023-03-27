//#![warn(clippy::unwrap_used, clippy::panic)]
#![allow(clippy::needless_update, unused_assignments)]

extern crate anyhow;
extern crate intuitive;
extern crate rustneat;

pub mod ai;
pub mod core;
pub mod ui;
pub mod utils;

use ai::stockfish::{self};
use anyhow::Result;

fn main() -> Result<()> {
    // run async main using only futures
    // futures::executor::block_on(run())?;

    let str = stockfish::eval("5B2/N2p2p1/2p5/2P1rP1R/3p1k1K/7B/4rP1P/8 w - - 0 1".to_string())?;

    // get last line
    let lines = str.lines();
    let last_line = lines.last().unwrap();

    // last line looks like this:
    // Final evaluation       +4.68 (white side) [with scaled NNUE
    // resolve to: 4.68
    // this CAN BE NEGATIVE

    let mut split = last_line.split_whitespace();
    let score = split.nth(2).unwrap();

    // score is +x.xx or -x.xx. resolve to f64
    let score = score.parse::<f64>()?;

    println!("Score: {}", score);

    Ok(())
}
