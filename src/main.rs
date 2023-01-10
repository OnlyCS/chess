#![warn(clippy::unwrap_used, clippy::panic)]
#![allow(clippy::needless_update)]

use intuitive::terminal::Terminal;
use crate::ui::root::Root;


pub mod game;
pub mod pieces;
pub mod types;
pub mod ui;
pub mod utils;

fn main() {
    let mut terminal = Terminal::new(Root::new()).expect("Failed to create UI");

    terminal.run().expect("Failed to run UI");
}
