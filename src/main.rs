#![warn(clippy::unwrap_used, clippy::panic)]
#![allow(clippy::needless_update, clippy::borrowed_box)]

use crate::ui::root::Root;
use intuitive::terminal::Terminal;

pub mod game;
pub mod parts;
pub mod pieces;
pub mod types;
pub mod ui;

fn main() {
    let mut terminal = Terminal::new(Root::new()).expect("Failed to create UI");

    terminal.run().expect("Failed to run UI");
}
