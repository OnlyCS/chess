use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counter<T: Copy + Eq + Add + Sub + AddAssign<u8> + SubAssign<u8>> {
    start: T,
    current: T,
}

impl<T: Copy + Eq + Add + Sub + AddAssign<u8> + SubAssign<u8>> Counter<T> {
    pub fn new(start: T) -> Self {
        Self {
            start,
            current: start,
        }
    }

    pub fn get(&self) -> T {
        self.current
    }

    pub fn reset(&mut self) {
        self.current = self.start;
    }

    pub fn inc(&mut self) {
        self.current += 1;
    }

    pub fn dec(&mut self) {
        self.current -= 1;
    }
}
