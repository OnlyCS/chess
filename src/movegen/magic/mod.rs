/// magic number generator
#[allow(unused)]
pub mod magic_numbers;

/// generated magic numbers
#[rustfmt::skip]
#[allow(warnings)]
pub mod generated_magics;

/// generated lookup tables
#[rustfmt::skip]
#[allow(warnings)]
#[cfg(not(feature = "gen-tables"))]
pub mod generated_lookups {
	// avoid rust-analyzer going bazonkers
	include!("generated_lookups.rs");
}

/// lookup tables generation
#[allow(unused)]
pub mod lookup_tables;

/// common functions
#[allow(unused)]
pub mod helper;

// everything in this submodule was read, understood, and refactored from
// *codfish* https://github.com/jsilll/codfish
// *Sebastian Lague* https://github.com/SebLague/Chess-Coding-Adventure
// *CPW* https://www.chessprogramming.org/
