use super::generated_magics::*;
use super::helper;
use crate::prelude::*;

pub fn generate_movement_masks(dirs: [(i8, i8); 4]) -> String {
    let arr_str = Square::every()
        .map(|sq| format!("{:#x},", helper::gen_movement_mask(sq, dirs)))
        .collect::<String>();

    format!("[{arr_str}]")
}

pub fn generate_attacks(rook: bool) -> String {
    let arr_str = Square::every()
        .map(|sq| create_table(sq, rook))
        .collect::<String>();

    format!("[{arr_str}]")
}

pub fn create_table(sq: Square, rook: bool) -> String {
    let dirs = if rook {
        helper::DIRS_ROOK
    } else {
        helper::DIRS_BISHOP
    };

    let movement_mask = helper::gen_movement_mask(sq, dirs);

    let bits_in = movement_mask.count_bits();
    let num_blocker_pats = 1u16 << bits_in;
    let magic = if rook {
        MAGICS_ROOK[sq as usize]
    } else {
        MAGICS_BISHOP[sq as usize]
    };

    let mut table = vec![0; num_blocker_pats as usize];

    for i in 0..num_blocker_pats {
        let i = i as usize;
        let pattern = movement_mask.set_occupancy(i, bits_in);

        let index = pattern.trimmed_mul(magic) >> (64 - bits_in);
        let attack_mask = helper::gen_attack_mask(sq, pattern, dirs);

        table[index as usize] = attack_mask;
    }

    let mut s = String::new();

    for attack in table.iter() {
        s.push_str(&format!("{attack:#x},"));
    }

    format!("&[{s}],")
}

pub fn save() {
    let mut file_str = format!(
        "
		// THIS FILE IS AUTOGENERATED. DO NOT EDIT.
		use crate::prelude::*;
		pub const MOVEMENTS_ROOK: [Bitboard; 64] = {};
		pub const MOVEMENTS_BISHOP: [Bitboard; 64] = {};
		pub const ATTACKS_ROOK: [&'static [Bitboard]; 64] = {};
		pub const ATTACKS_BISHOP: [&'static [Bitboard]; 64] = {};
	",
        generate_movement_masks(helper::DIRS_ROOK),
        generate_movement_masks(helper::DIRS_BISHOP),
        generate_attacks(true),
        generate_attacks(false),
    );

    // save to src/movegen/magic/generated_lookups.rs
    let mut file = std::fs::write("src/movegen/magic/generated_lookups.rs", file_str).unwrap();
}