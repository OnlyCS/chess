use std::error::Error;

pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }

    pub fn set(&mut self, x: usize, y: usize) -> Result<(), Box<dyn Error>> {
        if Coordinate::new(x, y).is_oob() {
            return Err("Coordinate out of bounds".into());
        }

        self.x = x;
        self.y = y;

        Ok(())
    }

    pub fn is_oob(&self) -> bool {
        self.x > 7 || self.y > 7
    }

    // shouldn't implicitly clone
    pub fn copy(&self) -> Self {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
