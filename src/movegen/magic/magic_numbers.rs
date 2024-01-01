use std::ffi::c_int;

use super::helper;
use crate::prelude::*;

#[rustfmt::skip]
const RELEVANT_BITS_ROOK: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

#[rustfmt::skip]
const RELEVANT_BITS_BISHOP: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6
];

// 0-dep rng
#[link(name = "c")]
extern "C" {
    fn srandom(seed: c_int);
    fn random() -> c_int;
}

fn dense_random() -> u64 {
    let n1 = unsafe { random() & 0xffff } as u64;
    let n2 = unsafe { random() & 0xffff } as u64;
    let n3 = unsafe { random() & 0xffff } as u64;
    let n4 = unsafe { random() & 0xffff } as u64;

    return n1 | (n2 << 16) | (n3 << 32) | (n4 << 48);
}

fn gen_candidate() -> u64 {
    dense_random() & dense_random() & dense_random()
}

fn gen_magic_number(
    sq: Square,
    relevant_bits: u8,
    moves_fn: fn(Square) -> Bitboard,
    attacks_fn: fn(Square, Bitboard) -> Bitboard,
) -> Option<u64> {
    let movement_mask = moves_fn(sq);
    let bits_in_mask = movement_mask.count_bits();
    let num_occupancies = 1u16 << bits_in_mask;

    let mut used_attacks = [0; 4096];

    let (occupancies, attacks): (Vec<_>, Vec<_>) = (0..num_occupancies)
        .into_par_iter()
        .map(|i| {
            let occupancy = movement_mask.set_occupancy(i as usize, bits_in_mask);
            let attack = attacks_fn(sq, occupancy);

            (occupancy, attack)
        })
        .unzip();

    for _ in 0..1_000_000_000 {
        let candidate = gen_candidate();
        let moves_magic = ((movement_mask as u128 * candidate as u128) & (0xff << 56)) as u64;

        if moves_magic.count_ones() < 6 {
            continue;
        }

        let mut ok = true;

        for i in 0..num_occupancies {
            let i = i as usize;

            let magic_idx = occupancies[i].trimmed_mul(candidate) >> (64 - relevant_bits);
            let magic_idx = magic_idx as usize;

            if used_attacks[magic_idx] == Bitboard::EMPTY {
                used_attacks[magic_idx] = attacks[i];
            } else if used_attacks[magic_idx] != attacks[i] {
                ok = false;
                break;
            }
        }

        if ok {
            return Some(candidate);
        }

        used_attacks = [0; 4096];
    }

    None
}

pub fn gen_magic_numbers() {
    let now = std::time::SystemTime::now();
    let unix = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    let seed = unix.subsec_nanos();

    unsafe {
        srandom(seed as i32);
    }

    println!("pub const MAGICS_ROOK: [u64; 64] = [");
    for sq in Square::every() {
        let magic_number = gen_magic_number(
            sq,
            RELEVANT_BITS_ROOK[sq as usize],
            |sq| helper::gen_movement_mask(sq, helper::DIRS_ROOK),
            |sq, filled| helper::gen_attack_mask(sq, filled, helper::DIRS_ROOK),
        );
        println!("    {:#x}u64,", magic_number.unwrap_or(0));
    }
    println!("];");

    println!("pub const MAGICS_BISHOP: [u64; 64] = [");
    for sq in Square::every() {
        let magic_number = gen_magic_number(
            sq,
            RELEVANT_BITS_BISHOP[sq as usize],
            |sq| helper::gen_movement_mask(sq, helper::DIRS_BISHOP),
            |sq, filled| helper::gen_attack_mask(sq, filled, helper::DIRS_BISHOP),
        );
        println!("    {:#x}u64,", magic_number.unwrap_or(0));
    }
    println!("];");
}
