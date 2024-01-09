use crate::prelude::*;

pub mod depth;
pub mod material;
pub mod tables;

pub trait StaticEvaluator: Copy + Send + Sync + 'static {
    fn eval(&self, position: &Position) -> f64;
}

pub use material::*;
pub use tables::*;

pub struct CompoundEvaluator;

impl CompoundEvaluator {
    pub fn eval(position: &Position) -> f64 {
        let mut score = 0.;

        score += MaterialEvaluator.eval(position) * 2.0;
        score += PieceTableEvaluator.eval(position) * 1.0;

        score
    }
}
