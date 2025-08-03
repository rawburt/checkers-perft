use crate::bitboard::Bitboard;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub movers: Bitboard,
    pub jumped: Bitboard,
}

impl Move {
    pub fn new(movers: Bitboard, jumped: Bitboard) -> Self {
        Move { movers, jumped }
    }
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Move {{ movers: {:?}, jumped: {:?} }}",
            self.movers.as_notation_vector(),
            self.jumped.as_notation_vector()
        )
    }
}
