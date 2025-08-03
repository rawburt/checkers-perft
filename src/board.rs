use std::fmt;

use crate::{bitboard::Bitboard, r#move::Move};

pub const PLAYABLE: u64 = 2130169298400;
pub const BLACK_START: u64 = 2130035343360;
pub const BLACK_KING_ROW: u64 = 2061584302080;
pub const WHITE_START: u64 = 261600;
pub const WHITE_KING_ROW: u64 = 480;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub bp: Bitboard,
    pub wp: Bitboard,
    pub kings: Bitboard,
}

impl Board {
    pub fn new() -> Self {
        Board {
            bp: Bitboard::from(BLACK_START),
            wp: Bitboard::from(WHITE_START),
            kings: Bitboard::new(),
        }
    }

    pub fn from_bitboards(bp: Bitboard, wp: Bitboard, kings: Bitboard) -> Self {
        Board {
            bp,
            wp,
            kings,
        }
    }

    pub fn movers(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => self.black_movers(),
            Color::White => self.white_movers(),
        }
    }

    pub fn moves(&self, color: Color, piece: Bitboard) -> Bitboard {
        match color {
            Color::Black => self.black_moves(piece),
            Color::White => self.white_moves(piece),
        }
    }

    pub fn jumpers(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => self.black_jumpers(),
            Color::White => self.white_jumpers(),
        }
    }

    pub fn jumps(&self, color: Color, piece: Bitboard) -> Bitboard {
        match color {
            Color::Black => self.black_jumps(piece),
            Color::White => self.white_jumps(piece),
        }
    }

    pub fn apply_move(&mut self, color: Color, m: &Move) {
        match color {
            Color::Black => {
                self.bp ^= m.movers;
                self.wp ^= m.jumped;
            }
            Color::White => {
                self.wp ^= m.movers;
                self.bp ^= m.jumped;
            }
        }
        if !(m.movers & self.kings).is_empty() {
            self.kings ^= m.movers;
        }
        if !(m.jumped & self.kings).is_empty() {
            self.kings ^= m.jumped & self.kings;
        }
    }

    pub fn promote_kings(&mut self) -> Bitboard {
        let black_kings = self.bp & WHITE_KING_ROW;
        let white_kings = self.wp & BLACK_KING_ROW;
        let new_kings = (black_kings | white_kings) & !self.kings;
        if !new_kings.is_empty() {
            self.kings |= new_kings;
        }
        return new_kings;
    }

    fn black_movers(&self) -> Bitboard {
        let empty = !(self.bp | self.wp) & PLAYABLE;
        let mut movers = (empty << 4) & self.bp;
        movers |= (empty << 5) & self.bp;
        let kings = self.bp & self.kings;
        if !kings.is_empty() {
            movers |= (empty >> 4) & kings;
            movers |= (empty >> 5) & kings;
        }
        return movers;
    }

    fn black_moves(&self, piece: Bitboard) -> Bitboard {
        let empty = !(self.bp | self.wp) & PLAYABLE;
        let mut moves = (piece >> 4 | piece >> 5) & empty;
        let king = self.kings & piece;
        if !king.is_empty() {
            moves |= (piece << 4 | piece << 5) & empty;
        }
        return moves;
    }

    fn black_jumpers(&self) -> Bitboard {
        let empty = !(self.bp | self.wp) & PLAYABLE;
        let mut jumped = (empty << 4) & self.wp;
        let mut jumpers = (jumped << 4) & self.bp;
        jumped = (empty << 5) & self.wp;
        jumpers |= (jumped << 5) & self.bp;
        let kings = self.bp & self.kings;
        if !kings.is_empty() {
            jumped = (empty >> 4) & self.wp;
            jumpers |= (jumped >> 4) & kings;
            jumped = (empty >> 5) & self.wp;
            jumpers |= (jumped >> 5) & kings;
        }
        return jumpers;
    }

    fn black_jumps(&self, piece: Bitboard) -> Bitboard {
        let empty = !(self.bp | self.wp) & PLAYABLE;
        let mut jumps = ((piece >> 4 & self.wp) >> 4) & empty;
        jumps |= ((piece >> 5 & self.wp) >> 5) & empty;
        let king = self.kings & piece;
        if !king.is_empty() {
            jumps |= ((piece << 4 & self.wp) << 4) & empty;
            jumps |= ((piece << 5 & self.wp) << 5) & empty;
        }
        return jumps;
    }

    fn white_movers(&self) -> Bitboard {
        let empty = !(self.bp | self.wp) & PLAYABLE;
        let mut movers = (empty >> 4) & self.wp;
        movers |= (empty >> 5) & self.wp;
        let kings = self.wp & self.kings;
        if !kings.is_empty() {
            movers |= (empty << 4) & kings;
            movers |= (empty << 5) & kings;
        }
        return movers;
    }

    fn white_moves(&self, piece: Bitboard) -> Bitboard {
        let empty = !(self.bp | self.wp) & PLAYABLE;
        let mut moves = (piece << 4 | piece << 5) & empty;
        let king = self.kings & piece;
        if !king.is_empty() {
            moves |= (piece >> 4 | piece >> 5) & empty;
        }
        return moves;
    }

    fn white_jumpers(&self) -> Bitboard {
        let empty = !(self.bp | self.wp) & PLAYABLE;
        let mut jumped = (empty >> 4) & self.bp;
        let mut jumpers = (jumped >> 4) & self.wp;
        jumped = (empty >> 5) & self.bp;
        jumpers |= (jumped >> 5) & self.wp;
        let kings = self.wp & self.kings;
        if !kings.is_empty() {
            jumped = (empty << 4) & self.bp;
            jumpers |= (jumped << 4) & kings;
            jumped = (empty << 5) & self.bp;
            jumpers |= (jumped << 5) & kings;
        }
        return jumpers;
    }

    fn white_jumps(&self, piece: Bitboard) -> Bitboard {
        let empty = !(self.bp | self.wp) & PLAYABLE;
        let mut jumps = ((piece << 4 & self.bp) << 4) & empty;
        jumps |= ((piece << 5 & self.bp) << 5) & empty;
        let king = self.kings & piece;
        if !king.is_empty() {
            jumps |= ((piece >> 4 & self.bp) >> 4) & empty;
            jumps |= ((piece >> 5 & self.bp) >> 5) & empty;
        }
        return jumps;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Board {{ black=[{:?}], white=[{:?}], kings=[{:?}] }}",
            self.bp.as_notation_vector(),
            self.wp.as_notation_vector(),
            self.kings.as_notation_vector()
        )
    }
}
