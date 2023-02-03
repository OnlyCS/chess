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
            _ => unreachable!(),
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

impl From<u8> for FileLetter {
    fn from(num: u8) -> Self {
        match num {
            0 => FileLetter::A,
            1 => FileLetter::B,
            2 => FileLetter::C,
            3 => FileLetter::D,
            4 => FileLetter::E,
            5 => FileLetter::F,
            6 => FileLetter::G,
            7 => FileLetter::H,
            _ => panic!("FileLetter cannot be created from a number greater than 7"),
        }
    }
}

impl Add<u8> for FileLetter {
    type Output = Self;

    fn add(self, other: u8) -> Self::Output {
        if other == 0 {
            return self;
        }

        match self {
            FileLetter::A => FileLetter::B,
            FileLetter::B => FileLetter::C,
            FileLetter::C => FileLetter::D,
            FileLetter::D => FileLetter::E,
            FileLetter::E => FileLetter::F,
            FileLetter::F => FileLetter::G,
            FileLetter::G => FileLetter::H,
            _ => panic!("FileLetter cannot be incremented past H"),
        };

        self + (other - 1)
    }
}

impl Sub<u8> for FileLetter {
    type Output = Self;

    fn sub(self, other: u8) -> Self::Output {
        if other == 0 {
            return self;
        }

        match self {
            FileLetter::H => FileLetter::G,
            FileLetter::G => FileLetter::F,
            FileLetter::F => FileLetter::E,
            FileLetter::E => FileLetter::D,
            FileLetter::D => FileLetter::C,
            FileLetter::C => FileLetter::B,
            FileLetter::B => FileLetter::A,
            _ => panic!("FileLetter cannot be decremented past A"),
        };

        self - (other - 1)
    }
}

impl PartialEq for FileLetter {
    fn eq(&self, other: &Self) -> bool {
        self == other
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

impl AddAssign<u8> for FileLetter {
    fn add_assign(&mut self, other: u8) {
        *self = self.clone() + other;
    }
}

impl SubAssign<u8> for FileLetter {
    fn sub_assign(&mut self, other: u8) {
        *self = self.clone() - other;
    }
}
