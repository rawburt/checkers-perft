use crate::translate::{bit_index_notation, notation_bit_index};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn new() -> Self {
        Bitboard(0)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn jumped(&self, destination: Bitboard) -> Bitboard {
        // assumes self and destination are single bit Bitboards
        let s = self.0.trailing_zeros();
        let d = destination.0.trailing_zeros();
        Bitboard(1 << ((s + d) / 2))
    }

    pub fn from_notation_vector(notation: &[u8]) -> Self {
        let mut bitboard = Bitboard::new();
        for &n in notation {
            let bit_index = notation_bit_index(n);
            bitboard.0 |= 1 << bit_index;
        }
        bitboard
    }

    pub fn as_notation_vector(&self) -> Vec<u8> {
        let mut notation = Vec::new();
        for piece in self {
            let bit_index = piece.trailing_zeros() as u8;
            notation.push(bit_index_notation(bit_index));
        }
        notation
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
}

pub struct BitboardIter {
    bits: u64,
}

impl Iterator for BitboardIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            None
        } else {
            let lsb = self.bits & self.bits.wrapping_neg();
            self.bits ^= lsb;
            Some(lsb)
        }
    }
}

impl IntoIterator for Bitboard {
    type Item = u64;
    type IntoIter = BitboardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIter { bits: self.0 }
    }
}

impl<'a> IntoIterator for &'a Bitboard {
    type Item = u64;
    type IntoIter = BitboardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIter { bits: self.0 }
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl std::ops::BitAnd<u64> for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 & rhs)
    }
}

impl std::ops::Shl<i32> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: i32) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl std::ops::Shr<i32> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: i32) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard(value)
    }
}

impl Into<u64> for Bitboard {
    fn into(self) -> u64 {
        self.0
    }
}
