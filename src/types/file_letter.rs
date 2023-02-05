#![allow(clippy::panic)]

use std::ops::{Add, AddAssign, Sub, SubAssign};

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

    pub fn inc(&self) -> Self {
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

    pub fn dec(&self) -> Self {
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
            FileLetter::A => 0,
            FileLetter::B => 1,
            FileLetter::C => 2,
            FileLetter::D => 3,
            FileLetter::E => 4,
            FileLetter::F => 5,
            FileLetter::G => 6,
            FileLetter::H => 7,
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

impl Add<u8> for FileLetter {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        let mut new_file = self;
        for _ in 0..rhs {
            new_file = new_file.inc();
        }
        new_file
    }
}

impl AddAssign<u8> for FileLetter {
    fn add_assign(&mut self, rhs: u8) {
        *self = self.clone() + rhs;
    }
}

impl Sub<u8> for FileLetter {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        let mut new_file = self;
        for _ in 0..rhs {
            new_file = new_file.dec();
        }
        new_file
    }
}

impl SubAssign<u8> for FileLetter {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.clone() - rhs;
    }
}
