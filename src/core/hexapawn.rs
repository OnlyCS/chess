use std::time::Duration;

use crate::grbl;
use anyhow::{bail, Result};
use serialport::SerialPortType;

use crate::{core::color::Color, utils::event_emitter::EventEmitter};

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

                // usize overflow protection
                if self.x != 0 {
                    if board
                        .at(self.x - 1, self.y + 1)
                        .map(|p| p.c == self.c.opposite())
                        .filter(|x| *x)
                        .is_some()
                    {
                        moves.push((self.x, self.y, self.x - 1, self.y + 1, true));
                    }
                }

                moves
            }
            Color::White => {
                let mut moves = vec![];

                // usize overflow protection
                if self.y != 0 {
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

                    // usize overflow protection
                    if self.x != 0 {
                        if board
                            .at(self.x - 1, self.y - 1)
                            .map(|p| p.c == self.c.opposite())
                            .filter(|x| *x)
                            .is_some()
                        {
                            moves.push((self.x, self.y, self.x - 1, self.y - 1, true));
                        }
                    }
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

            let port = serialport::new(port_name, 115200)
                .timeout(Duration::from_secs(5))
                .open()
                .expect("Failed to open Arduino Uno");

            grbl::init(port);

            emitter.on(Event::Move, |arg: (usize, usize, usize, usize, bool)| {
                let (tda_x1, tda_y1, tda_x2, tda_y2, capture) = arg;

                // all values are currently 2d-array (tda) values, ranging from (0,0) to (2,2), with the origin at the top-left
                // we need to fix this, with a minimum of 0.5 and a max of 2.5, and the origin at the bottom-left

                let x1 = tda_x1 as f64 + 0.5;
                let y1 = 2.5 - tda_y1 as f64;

                let x2 = tda_x2 as f64 + 0.5;
                let y2 = 2.5 - tda_y2 as f64;

                if capture {
                    grbl::goto((x2, y2)).unwrap();

                    // todo: electromangnet on

                    // go into on-the-line position
                    grbl::down_half().unwrap();
                    grbl::left_half().unwrap();

                    // always on-the-line
                    grbl::goto((0.0, 1.0)).unwrap();

                    // go to the left-side center
                    grbl::up_half().unwrap();

                    // move left 1, into captured pieces zone
                    grbl::left_half().unwrap();

                    // todo: electromagnet off

                    // go to (0,0)
                    grbl::origin().unwrap();
                }

                // go to the first piece
                grbl::goto((x1, y1)).unwrap();

                // todo: electromagnet on

                // go into on-the-line position
                grbl::down_half().unwrap();
                grbl::left_half().unwrap();

                // always on-the-line, go to the second piece
                grbl::goto((x2 - 0.5, y2 - 0.5)).unwrap();

                // go half-up, half-right
                grbl::up_half().unwrap();
                grbl::right_half().unwrap();

                // todo: electromagnet off

                // go to (0,0)
                grbl::origin().unwrap();
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
        if !(0..=2).contains(&x) || !(0..=2).contains(&y) {
            return None;
        }

        self.board[y][x].as_ref()
    }

    pub fn is_win(&self) -> Option<Color> {
        if self
            .get_moves()
            .iter()
            .filter(|(x1, y1, _, _, _)| self.at(*x1, *y1).unwrap().c == self.turn)
            .count()
            == 0
        {
            Some(self.turn.opposite())
        } else if let Some(p) = self
            .board
            .iter()
            .flatten()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap())
            .filter(|p| {
                p.y == match p.c {
                    Color::White => 0,
                    Color::Black => 2,
                }
            })
            .next()
        {
            Some(p.c)
        } else {
            None
        }
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

    pub fn make_move(&mut self, m: (usize, usize, usize, usize, bool)) -> Result<()> {
        if !(0..=2).contains(&m.0)
            || !(0..=2).contains(&m.1)
            || !(0..=2).contains(&m.2)
            || !(0..=2).contains(&m.3)
        {
            bail!("oob idiot")
        }

        self.board[m.3][m.2] = self.board[m.1][m.0].clone();
        self.board[m.1][m.0] = None;
        self.board[m.3][m.2].as_mut().unwrap().make_move(m);

        Ok(())
    }
}

impl Default for HexapawnBoard {
    fn default() -> Self {
        Self::new(false)
    }
}
