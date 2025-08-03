use crate::bitboard::Bitboard;
use crate::board::{Board, Color};
use crate::r#move::Move;

pub struct MoveGenerator {
    board: Board,
    color: Color,
}

impl MoveGenerator {
    pub fn new(board: Board, color: Color) -> Self {
        MoveGenerator { board, color }
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let jumpers = self.board.jumpers(self.color);
        if jumpers.is_empty() {
            self.generate_simple_moves()
        } else {
            self.generate_jump_moves(jumpers)
        }
    }

    fn generate_simple_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        let movers = self.board.movers(self.color);
        for mover in movers {
            let mover_bb = Bitboard::from(mover);
            let possible_moves = self.board.moves(self.color, mover_bb);
            for possible_move in possible_moves {
                let jumped = Bitboard::new();
                let possible_move_bb = Bitboard::from(possible_move);
                moves.push(Move::new(mover_bb | possible_move_bb, jumped));
            }
        }
        moves
    }

    fn generate_jump_moves(&self, jumpers: Bitboard) -> Vec<Move> {
        let mut jumps = Vec::new();
        for jumper in jumpers {
            let jumper_bb = Bitboard::from(jumper);
            let possible_jumps = self.board.jumps(self.color, jumper_bb);
            for destination in possible_jumps {
                let destination_bb = Bitboard::from(destination);
                let jumped_bb = jumper_bb.jumped(destination_bb);
                let current_move = Move::new(jumper_bb | destination_bb, jumped_bb);

                let mut new_board = self.board.clone();
                new_board.apply_move(self.color, &current_move);

                let multiple_jumps = new_board.jumps(self.color, destination_bb);

                if multiple_jumps.is_empty() {
                    jumps.push(current_move);
                } else {
                    let move_generator = MoveGenerator::new(new_board, self.color);
                    let further_jumps = move_generator.generate_jump_moves(destination_bb);
                    for further_jump in further_jumps {
                        let combined_move = Move::new(
                            current_move.movers ^ further_jump.movers,
                            current_move.jumped | further_jump.jumped,
                        );
                        jumps.push(combined_move);
                    }
                }
            }
        }
        jumps
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        bitboard::Bitboard,
        board::{Board, Color},
        move_generator::MoveGenerator,
        r#move::Move,
    };

    #[test]
    fn test_movegen_multi_jump() {
        let bp = Bitboard::from_notation_vector(&[23, 15, 8]);
        let wp = Bitboard::from_notation_vector(&[27]);
        let kings = Bitboard::from(0);
        let board = Board::from_bitboards(bp, wp, kings);
        let moves = MoveGenerator::new(board, Color::White).generate_moves();
        assert!(moves.len() == 1);
        let m = Move::new(
            Bitboard::from_notation_vector(&[27, 4]),
            Bitboard::from_notation_vector(&[23, 15, 8]),
        );
        assert!(moves.contains(&m));
    }

    #[test]
    fn test_movegen_circle_jump() {
        let board = Board::from_bitboards(
            Bitboard::from_notation_vector(&[23]),
            Bitboard::from_notation_vector(&[18, 19, 10, 11]),
            Bitboard::from_notation_vector(&[23]),
        );
        let moves = MoveGenerator::new(board, Color::Black).generate_moves();
        assert!(moves.len() == 2);
        let m = Move::new(
            Bitboard::new(),
            Bitboard::from_notation_vector(&[18, 19, 10, 11]),
        );
        assert!(moves.contains(&m));
    }

    #[test]
    fn test_movegen_black_kings() {
        let board = Board::from_bitboards(
            Bitboard::from_notation_vector(&[29]),
            Bitboard::new(),
            Bitboard::from_notation_vector(&[29]),
        );
        let moves = MoveGenerator::new(board, Color::Black).generate_moves();
        assert!(
            moves.len() == 1,
            "expected 1 move, found: {:?}",
            moves.len()
        );
        let m = Move::new(Bitboard::from_notation_vector(&[29, 25]), Bitboard::new());
        assert!(
            moves.contains(&m),
            "expected: {:?} in moves: {:?}",
            m,
            moves
        );
    }
}
