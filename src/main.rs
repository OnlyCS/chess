#![feature(const_for, const_trait_impl, const_mut_refs, const_option, let_chains)]

mod board;
mod gui;
mod movegen;
mod prelude;

fn main() {
    gui::run().unwrap();
}
