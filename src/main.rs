#![allow(internal_features)]
#![feature(
    const_for,
    const_trait_impl,
    const_mut_refs,
    const_option,
    let_chains,
    core_intrinsics
)]

use std::{sync::Arc, time::Instant};

use atomicfloat::AtomicF64;
use board::position::Position;

mod board;
mod evaluation;
mod gui;
mod movegen;
mod prelude;
mod rng;

fn main() {
    rng::init();
    //gui::run().unwrap();

    let mut gametree =
        evaluation::depth::GameTreeRoot::new(Position::new(), Arc::new(AtomicF64::new(0.)));

    let timer = Instant::now();

    gametree.populate(5);

    let time = timer.elapsed();

    let mut leaves = vec![];
    gametree.leaves(&mut leaves);
    let count = leaves.len();

    println!("populated {count} nodes in {} secs", time.as_secs_f32());
    println!(
        "thats about {} nodes per second!",
        count as f32 / time.as_secs_f32()
    );
    println!("attempting to populate remaining nodes");

    let mut count = 0;
    for l in &mut leaves {
        l.populate(1);
        count += 1;

        if count % 10_000 == 0 {
            println!("populated {} nodes", count);
        }
    }

    println!("done!")
}
