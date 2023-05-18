use std::time::Duration;

use serialport::SerialPortType;

use crate::{
    core::{color::Color, piece_move::Move},
    utils::event_emitter::EventEmitter,
};

#[derive(Clone)]
pub struct HexPiece {
    pub x: usize,
    pub y: usize,
    pub c: Color,
}

impl HexPiece {
    pub fn new(x: usize, y: usize, c: Color) -> Self {
        Self { x, y, c }
    }

    pub fn get_moves(&self, board: &HexapawnBoard) -> Vec<(usize, usize, usize, usize, bool)> {
        match self.c {
            Color::Black => {
                let mut moves = vec![];

                if board.at(self.x, self.y + 1).is_none() {
                    moves.push((self.x, self.y, self.x, self.y + 1, false));
                }

                if board
                    .at(self.x + 1, self.y + 1)
                    .map(|p| p.c == self.c.opposite())
                    .filter(|x| *x)
                    .is_some()
                {
                    moves.push((self.x, self.y, self.x + 1, self.y + 1, true));
                }

                if board
                    .at(self.x - 1, self.y + 1)
                    .map(|p| p.c == self.c.opposite())
                    .filter(|x| *x)
                    .is_some()
                {
                    moves.push((self.x, self.y, self.x - 1, self.y + 1, true));
                }

                moves
            }
            Color::White => {
                let mut moves = vec![];

                if board.at(self.x, self.y - 1).is_none() {
                    moves.push((self.x, self.y, self.x, self.y - 1, false));
                }

                if board
                    .at(self.x + 1, self.y - 1)
                    .map(|p| p.c == self.c.opposite())
                    .filter(|x| *x)
                    .is_some()
                {
                    moves.push((self.x, self.y, self.x + 1, self.y - 1, true));
                }

                if board
                    .at(self.x - 1, self.y - 1)
                    .map(|p| p.c == self.c.opposite())
                    .filter(|x| *x)
                    .is_some()
                {
                    moves.push((self.x, self.y, self.x - 1, self.y - 1, true));
                }

                moves
            }
        }
    }

    pub fn make_move(&mut self, m: (usize, usize, usize, usize, bool)) {
        self.x = m.2;
        self.y = m.3;
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub enum Event {
    #[default]
    Move,
}

#[derive(Clone)]
pub struct HexapawnBoard {
    board: Vec<Vec<Option<HexPiece>>>,
    pub event_emitter: EventEmitter<Event>,
    pub turn: Color,
}

impl HexapawnBoard {
    pub fn new(serial: bool) -> Self {
        let mut emitter = EventEmitter::default();

        if serial {
            emitter.on(Event::Move, |arg: Move| {
                let ports = serialport::available_ports().expect("No ports found");
                let mut port_name = Default::default();

                for p in ports {
                    if let SerialPortType::UsbPort(usb) = p.port_type {
                        if let Some(name) = usb.product {
                            if name.to_lowercase().contains("uno") {
                                port_name = p.port_name;
                                break;
                            }
                        }
                    }
                }

                if port_name == String::default() {
                    panic!("Failed to find Arduino Uno");
                }

                let mut port = serialport::new(port_name, 9600)
                    .timeout(Duration::from_secs(5))
                    .open()
                    .expect("Failed to open Arduino Uno");

                let for_arduino = arg.to_ard();
                let output = for_arduino.as_bytes();

                port.write_all(output)
                    .expect("Failed to write to Arduino Uno");

                let mut input = Default::default();

                port.read_to_string(&mut input)
                    .expect("Failed to read Ok from Arduino Uno");

                assert!(
                    input == "ok",
                    "Arduino Uno failed to respond with Ok, instead responded with {}",
                    input
                );

                drop(port);
            });
        }

        use Color::Black as B;
        use Color::White as W;

        let board = Self {
            board: vec![
                vec![
                    Some(HexPiece::new(0, 0, B)),
                    Some(HexPiece::new(1, 0, B)),
                    Some(HexPiece::new(2, 0, B)),
                ],
                vec![None, None, None],
                vec![
                    Some(HexPiece::new(0, 2, W)),
                    Some(HexPiece::new(1, 2, W)),
                    Some(HexPiece::new(2, 2, W)),
                ],
            ],
            turn: Color::White,
            event_emitter: emitter,
        };

        board
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&HexPiece> {
        self.board[y][x].as_ref()
    }

    pub fn get_moves(&self) -> Vec<(usize, usize, usize, usize, bool)> {
        let mut m = vec![];

        for p in self.board.iter().flatten() {
            if p.is_some() {
                m.extend(p.as_ref().unwrap().get_moves(self));
            }
        }

        m
    }

    pub fn make_move(&mut self, m: (usize, usize, usize, usize, bool)) {
        self.board[m.3][m.2] = self.board[m.1][m.0].clone();
        self.board[m.1][m.0] = None;
        self.board[m.3][m.2].as_mut().unwrap().make_move(m);
    }
}
