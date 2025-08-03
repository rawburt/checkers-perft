use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::bitboard::Bitboard;
use crate::r#move::Move;

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MoveKind {
    #[serde(rename = "simple")]
    Simple,
    #[serde(rename = "jump")]
    Jump,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveDetail {
    pub kind: MoveKind,
    pub moves: Vec<u8>,
}

impl MoveDetail {
    pub fn into_move(self) -> Move {
        match self.kind {
            MoveKind::Simple => self.from_simple(),
            MoveKind::Jump => self.from_jumps(),
        }
    }

    fn from_simple(self) -> Move {
        Move::new(Bitboard::from_notation_vector(&self.moves), Bitboard::new())
    }

    fn from_jumps(self) -> Move {
        let len = self.moves.len();
        let dest = self.moves[len - 1];
        let start = self.moves[0];
        let movers = Bitboard::from_notation_vector(&[start, dest]);
        let mut jumpers = Bitboard::new();
        let mut last: Option<Bitboard> = None;
        for n in self.moves {
            let n_bb = Bitboard::from_notation_vector(&[n]);
            if let Some(last_bb) = last {
                jumpers |= last_bb.jumped(n_bb);
            }
            last = Some(n_bb);
        }
        Move::new(movers, jumpers)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovePair {
    pub number: u32,
    pub black: MoveDetail,
    pub white: Option<MoveDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub headers: Vec<Header>,
    pub moves: Vec<MovePair>,
}

impl Game {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, serde_json::Error> {
        let data = fs::read_to_string(path).expect("Unable to read file");
        serde_json::from_str(&data)
    }
}

pub fn parse_games_from_json_array_file<P: AsRef<Path>>(path: P) -> Result<Vec<Game>, serde_json::Error> {
    let data = fs::read_to_string(path).expect("Unable to read file");
    serde_json::from_str(&data)
}
