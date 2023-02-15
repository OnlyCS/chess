#![allow(clippy::panic)]

use std::error::Error;

pub enum FileLetter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl FileLetter {
    pub fn vec_all() -> Vec<FileLetter> {
        vec![
            FileLetter::A,
            FileLetter::B,
            FileLetter::C,
            FileLetter::D,
            FileLetter::E,
            FileLetter::F,
            FileLetter::G,
            FileLetter::H,
        ]
    }

    pub fn next(&self) -> Result<Self, Box<dyn Error>> {
        match self {
            FileLetter::A => Ok(FileLetter::B),
            FileLetter::B => Ok(FileLetter::C),
            FileLetter::C => Ok(FileLetter::D),
            FileLetter::D => Ok(FileLetter::E),
            FileLetter::E => Ok(FileLetter::F),
            FileLetter::F => Ok(FileLetter::G),
            FileLetter::G => Ok(FileLetter::H),
            FileLetter::H => Err("Cannot go forward from H".into()),
        }
    }

    pub fn next_loop(&self) -> Self {
        match self {
            FileLetter::A => FileLetter::B,
            FileLetter::B => FileLetter::C,
            FileLetter::C => FileLetter::D,
            FileLetter::D => FileLetter::E,
            FileLetter::E => FileLetter::F,
            FileLetter::F => FileLetter::G,
            FileLetter::G => FileLetter::H,
            FileLetter::H => FileLetter::A,
        }
    }

    pub fn prev(&self) -> Result<Self, Box<dyn Error>> {
        match self {
            FileLetter::A => Err("Cannot go back from A".into()),
            FileLetter::B => Ok(FileLetter::A),
            FileLetter::C => Ok(FileLetter::B),
            FileLetter::D => Ok(FileLetter::C),
            FileLetter::E => Ok(FileLetter::D),
            FileLetter::F => Ok(FileLetter::E),
            FileLetter::G => Ok(FileLetter::F),
            FileLetter::H => Ok(FileLetter::G),
        }
    }

    pub fn prev_loop(&self) -> Self {
        match self {
            FileLetter::A => FileLetter::H,
            FileLetter::B => FileLetter::A,
            FileLetter::C => FileLetter::B,
            FileLetter::D => FileLetter::C,
            FileLetter::E => FileLetter::D,
            FileLetter::F => FileLetter::E,
            FileLetter::G => FileLetter::F,
            FileLetter::H => FileLetter::G,
        }
    }
}

impl From<FileLetter> for char {
    fn from(file: FileLetter) -> Self {
        match file {
            FileLetter::A => 'a',
            FileLetter::B => 'b',
            FileLetter::C => 'c',
            FileLetter::D => 'd',
            FileLetter::E => 'e',
            FileLetter::F => 'f',
            FileLetter::G => 'g',
            FileLetter::H => 'h',
        }
    }
}

impl From<char> for FileLetter {
    fn from(letter: char) -> Self {
        match letter {
            'a' => FileLetter::A,
            'b' => FileLetter::B,
            'c' => FileLetter::C,
            'd' => FileLetter::D,
            'e' => FileLetter::E,
            'f' => FileLetter::F,
            'g' => FileLetter::G,
            'h' => FileLetter::H,
            _ => panic!("FileLetter cannot be created from a character other than a-h"),
        }
    }
}

impl From<FileLetter> for u8 {
    fn from(val: FileLetter) -> Self {
        match val {
            FileLetter::A => 1,
            FileLetter::B => 2,
            FileLetter::C => 3,
            FileLetter::D => 4,
            FileLetter::E => 5,
            FileLetter::F => 6,
            FileLetter::G => 7,
            FileLetter::H => 8,
        }
    }
}

impl From<u8> for FileLetter {
    fn from(val: u8) -> Self {
        match val {
            1 => FileLetter::A,
            2 => FileLetter::B,
            3 => FileLetter::C,
            4 => FileLetter::D,
            5 => FileLetter::E,
            6 => FileLetter::F,
            7 => FileLetter::G,
            8 => FileLetter::H,
            _ => panic!("FileLetter cannot be created from a number other than 1-8"),
        }
    }
}

impl PartialEq for FileLetter {
    fn eq(&self, other: &Self) -> bool {
        let self_num: u8 = Into::into(self.clone());
        let other_num: u8 = Into::into(other.clone());

        self_num == other_num
    }
}

impl PartialOrd for FileLetter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_num: u8 = Into::into(self.clone());
        let other_num: u8 = Into::into(other.clone());

        self_num.partial_cmp(&other_num)
    }
}

impl Clone for FileLetter {
    fn clone(&self) -> Self {
        match self {
            FileLetter::A => FileLetter::A,
            FileLetter::B => FileLetter::B,
            FileLetter::C => FileLetter::C,
            FileLetter::D => FileLetter::D,
            FileLetter::E => FileLetter::E,
            FileLetter::F => FileLetter::F,
            FileLetter::G => FileLetter::G,
            FileLetter::H => FileLetter::H,
        }
    }
}
