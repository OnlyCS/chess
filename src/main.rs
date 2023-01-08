#![warn(clippy::unwrap_used, clippy::panic)]

use game::game_manager::GameManager;

pub mod game;
pub mod pieces;
pub mod types;
pub mod ui;
pub mod utils;

fn main() {
    let game = GameManager::new();

    // flat_iter() the game.board and print, with a new ln every 8

    game.board.flat_iter().enumerate().for_each(|(i, piece)| {
        print!("{} ", piece);
        if i % 8 == 7 {
            println!();
        }
    });
}
