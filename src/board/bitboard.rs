use crate::prelude::*;

// goofy
const DEBRUIJN64: u64 = 0x03f79d71b4cb0a89;
#[rustfmt::skip]
const INDEX64: [u8; 64] = [
    0,  1, 48,  2, 57, 49, 28,  3,
   61, 58, 50, 42, 38, 29, 17,  4,
   62, 55, 59, 36, 53, 51, 43, 22,
   45, 39, 33, 30, 24, 18, 12,  5,
   63, 47, 56, 27, 60, 41, 37, 16,
   54, 35, 52, 21, 44, 32, 23, 11,
   46, 26, 40, 15, 34, 20, 31, 10,
   25, 14, 19,  9, 13,  8,  7,  6
];

pub trait BitboardU64 {
    const EMPTY: Self;

    fn clear_bit(&mut self, idx: Square);
    fn pop_bit(&mut self) -> Square;
    fn count_bits(&self) -> u8;
    fn at(&self, idx: Square) -> bool;
    fn set(&mut self, idx: Square);
    fn unset(&mut self, idx: Square);
    fn last_bit(&self) -> Square;
    fn set_occupancy(&self, index: usize, bits_in_mask: u8) -> Self;
    fn trimmed_mul(&self, other: Self) -> Self;
    fn pretty(&self) -> String {
        let mut s = String::from("\n  a b c d e f g h\n");

        for rank in (0..8u8).rev() {
            for file in 0..8u8 {
                let idx = rank * 8 + file;

                if idx % 8 == 0 {
                    s.push_str(&format!("{} ", rank + 1));
                }

                if self.at(idx) {
                    s.push_str("X ");
                } else {
                    s.push_str(". ");
                }

                if idx % 8 == 7 {
                    s.push_str(&format!("{}\n", rank + 1));
                }
            }
        }

        s.push_str("  a b c d e f g h\n");

        return s;
    }
}

/// Little-endian rank-file mapping.
/// See the [Chess Programming Wiki](https://www.chessprogramming.org/Square_Mapping_Considerations#Little-Endian_Rank-File_Mapping).
pub type Bitboard = u64;

impl const BitboardU64 for Bitboard {
    const EMPTY: Bitboard = 0;

    fn clear_bit(&mut self, idx: u8) {
        *self &= !(1 << idx);
    }

    fn pop_bit(&mut self) -> Square {
        let sq = self.last_bit();
        self.clear_bit(sq);

        return sq;
    }

    fn count_bits(&self) -> u8 {
        return self.count_ones() as u8;
    }

    fn at(&self, idx: u8) -> bool {
        return self & (1 << idx) != 0;
    }

    fn set(&mut self, idx: u8) {
        *self |= 1 << idx;
    }

    fn unset(&mut self, idx: u8) {
        *self &= !(1 << idx);
    }

    fn last_bit(&self) -> u8 {
        INDEX64[(((self & !self.wrapping_sub(1)).wrapping_mul(DEBRUIJN64)) >> 58) as usize]
    }

    fn trimmed_mul(&self, other: Self) -> Self {
        let thisu128 = *self as u128;
        let otheru128 = other as u128;
        let product = thisu128 * otheru128;
        let trim64 = product & u64::MAX as u128;

        return trim64 as u64;
    }

    fn set_occupancy(&self, index: usize, bits_in_mask: u8) -> Bitboard {
        let mut this = *self;
        let mut occupancy = 0u64;

        for bit in 0..bits_in_mask {
            let popped = this.pop_bit();

            if index & (1 << bit) != 0 {
                occupancy |= popped.to_bitboard();
            }
        }

        return occupancy;
    }
}
