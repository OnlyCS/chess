use crate::prelude::*;

pub mod gametree;
pub mod material;

pub trait StaticEvaluator: Copy + Send + Sync + 'static {
    fn eval(&self, position: &Position) -> f64;
}

pub use material::*;
