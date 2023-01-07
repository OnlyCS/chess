pub struct ScoreCounter {
    score: i32,
}

impl ScoreCounter {
    pub fn new() -> ScoreCounter {
        ScoreCounter { score: 0 }
    }

    pub fn get(&self) -> i32 {
        self.score
    }

    pub fn add(&mut self, score: i32) {
        self.score += score;
    }

    pub fn sub(&mut self, score: i32) {
        self.score -= score;
    }

    pub fn reset(&mut self) {
        self.score = 0;
    }
}

impl Default for ScoreCounter {
    fn default() -> Self {
        Self::new()
    }
}