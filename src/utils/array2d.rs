pub struct Array2D<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Array2D<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Array2D<T> {
        Array2D {
            width,
            height,
            data,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.data[x + y * self.width])
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&mut self.data[x + y * self.width])
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if let Some(cell) = self.get_mut(x, y) {
            *cell = value;
        }
    }

    pub fn flat_iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn to_vec(&self) -> Vec<Vec<&T>> {
        let array = self
            .data
            .chunks(self.width)
            .into_iter()
            .collect::<Vec<&[T]>>();

        let mut vec = Vec::new();

        for row in array {
            let mut row_vec = Vec::new();

            for cell in row {
                row_vec.push(cell);
            }

            vec.push(row_vec);
        }

        vec
    }

    pub fn to_vec_mut(&mut self) -> Vec<Vec<&mut T>> {
        let array = self
            .data
            .chunks_mut(self.width)
            .into_iter()
            .collect::<Vec<&mut [T]>>();

        let mut vec = Vec::new();

        for row in array {
            let mut row_vec = Vec::new();

            for cell in row {
                row_vec.push(cell);
            }

            vec.push(row_vec);
        }

        vec
    }

    pub fn iter(&self) -> impl Iterator<Item = Vec<&T>> {
        self.to_vec().into_iter()
    }
}

impl<T> Default for Array2D<T> {
    fn default() -> Self {
        Array2D {
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }
}