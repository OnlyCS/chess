//#![warn(clippy::unwrap_used, clippy::panic)]
#![allow(clippy::needless_update, unused_assignments)]

extern crate anyhow;
extern crate futures;
extern crate intuitive;
extern crate rustneat;
extern crate zip;

pub mod ai;
pub mod core;
pub mod ui;
pub mod utils;

use ai::stockfish::{self};
use anyhow::Result;

// async fn run() -> Result<()> {
//     // let mut t = Terminal::new(Root::new()).await?;
//     // t.run().await?;

//     // Ok(())

//     // download stockfish
//     stockfish::download()?;
//     stockfish::extract()?;

//     println!("Stockfish downloaded and extracted successfully.");

//     let mut process = StockfishProcess::new()?;

//     Ok(())
// }

fn main() -> Result<()> {
    // run async main using only futures
    // futures::executor::block_on(run())?;

    println!("{}", stockfish::rw("uci")?);

    Ok(())
}
