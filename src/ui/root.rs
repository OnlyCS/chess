use crossterm::terminal::size;
use intuitive::components::*;
use intuitive::*;

use crate::game::game_manager::GameManager;
use crate::ui::tile::Tile;

use super::with_size::{board_flex, get_instructions};

#[component(Root)]
pub fn render() {
    let game = GameManager::new();

    let key_handler = on_key! {
        KeyEvent { code: Char('q'), .. } => event::quit(),
    };

    let size = size().expect("Failed to get terminal size");

    let instructions = get_instructions(size);
    let bflex = board_flex(size);

    render! {
        VStack(on_key: key_handler, flex: [bflex.2, bflex.3]) {
            HStack(on_key: key_handler, flex: [bflex.0, bflex.1]) {
                Section(title: "Board") {
                    VStack() {
                        HStack() {
                            Tile(piece_str: game.board.get(0, 0).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(1, 0).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(2, 0).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(3, 0).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(4, 0).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(5, 0).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(6, 0).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(7, 0).expect("Failed to get board").to_string())
                        }
                        HStack() {
                            Tile(piece_str: game.board.get(0, 1).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(1, 1).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(2, 1).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(3, 1).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(4, 1).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(5, 1).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(6, 1).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(7, 1).expect("Failed to get board").to_string())
                        }
                        HStack() {
                            Tile(piece_str: game.board.get(0, 2).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(1, 2).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(2, 2).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(3, 2).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(4, 2).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(5, 2).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(6, 2).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(7, 2).expect("Failed to get board").to_string())
                        }
                        HStack() {
                            Tile(piece_str: game.board.get(0, 3).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(1, 3).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(2, 3).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(3, 3).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(4, 3).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(5, 3).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(6, 3).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(7, 3).expect("Failed to get board").to_string())
                        }
                        HStack() {
                            Tile(piece_str: game.board.get(0, 4).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(1, 4).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(2, 4).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(3, 4).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(4, 4).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(5, 4).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(6, 4).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(7, 4).expect("Failed to get board").to_string())
                        }
                        HStack() {
                            Tile(piece_str: game.board.get(0, 5).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(1, 5).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(2, 5).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(3, 5).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(4, 5).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(5, 5).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(6, 5).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(7, 5).expect("Failed to get board").to_string())
                        }
                        HStack() {
                            Tile(piece_str: game.board.get(0, 6).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(1, 6).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(2, 6).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(3, 6).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(4, 6).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(5, 6).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(6, 6).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(7, 6).expect("Failed to get board").to_string())
                        }
                        HStack() {
                            Tile(piece_str: game.board.get(0, 7).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(1, 7).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(2, 7).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(3, 7).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(4, 7).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(5, 7).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(6, 7).expect("Failed to get board").to_string())
                            Tile(piece_str: game.board.get(7, 7).expect("Failed to get board").to_string())
                        }
                    }
                }

                Section(title: "Instructions") {
                    Centered() {
                        VStack() {
                            Text(text: instructions)
                        }
                    }
                }
            }
            Section(title: "e") {}
        }
    }
}
