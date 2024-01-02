/// magic number generator, and the pre-generated magic numbers
pub mod magic;

/// sliding piece movements
mod sliders;

/// pawn movements
mod pawns;

/// non-sliding piece movements
mod non_sliders;

pub use non_sliders::*;
pub use pawns::*;
pub use sliders::*;
