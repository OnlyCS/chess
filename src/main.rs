#![feature(const_for, const_trait_impl, const_mut_refs, const_option, let_chains)]

mod board;
mod evaluation;
mod gui;
mod movegen;
mod prelude;
mod rng;

fn main() {
    rng::init();
    gui::run().unwrap();
}
