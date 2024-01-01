#![feature(
    const_for,
    const_trait_impl,
    const_mut_refs,
    const_option,
    const_closures,
    let_chains
)]
#![allow(incomplete_features)]
#![allow(warnings)]

mod board;
mod movegen;
mod prelude;

fn main() {
    println!("{:?}", movegen::magic::generator::gen_magic_numbers());
}
