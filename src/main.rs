#![feature(
    const_for,
    const_trait_impl,
    const_mut_refs,
    const_option,
    const_closures,
    let_chains
)]
#![allow(incomplete_features)]

mod board;
mod movegen;
mod prelude;

fn main() {
    movegen::magic::lookup_tables::save();
}
