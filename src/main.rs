#![warn(clippy::unwrap_used, clippy::panic)]
#![allow(clippy::needless_update, unused_assignments)]

extern crate anyhow;
extern crate intuitive;
extern crate pleco;
extern crate rustneat;

pub mod core;
pub mod ui;
pub mod utils;

use anyhow::Result;
use intuitive::terminal::Terminal;
use ui::root::Root;

fn main() -> Result<()> {
    let mut t = Terminal::new(Root::new())?;
    t.run()?;

    Ok(())
}
