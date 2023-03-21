#![warn(clippy::unwrap_used, clippy::panic)]
#![allow(clippy::needless_update, clippy::borrowed_box)]

extern crate intuitive;
extern crate pleco;
extern crate rustneat;

pub mod core;
pub mod ui;
pub mod utils;

use std::error::Error;

// use crate::ui::root::Root;
use intuitive::terminal::Terminal;

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal::new(Root::new())?.run()?;

    Ok(())
}
