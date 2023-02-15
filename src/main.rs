#![warn(clippy::unwrap_used, clippy::panic)]
#![allow(clippy::needless_update, clippy::borrowed_box)]

extern crate intuitive;
extern crate pleco;
extern crate rustneat;

pub mod game;
pub mod parts;
pub mod pieces;
pub mod types;
pub mod ui;

use std::error::Error;

use crate::ui::root::Root;
use intuitive::terminal::Terminal;

fn main() -> Result<(), Box<dyn Error>> {
    Terminal::new(Root::new())?.run()?;

    Ok(())
}
