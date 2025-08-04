use crate::{
    board::{Board, Color},
    move_generator::MoveGenerator,
};
use rayon::prelude::*;

fn perft_recur(color: Color, board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let move_generator = MoveGenerator::new(board.clone(), color);
    let moves = move_generator.generate_moves();
    let mut count = 0;

    for m in moves {
        let mut new_board = board.clone();
        new_board.apply_move(color, &m);
        new_board.promote_kings();
        count += perft_recur(color.opposite(), &new_board, depth - 1);
    }

    count
}

pub fn perft(color: Color, board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let move_generator = MoveGenerator::new(board.clone(), color);
    let moves = move_generator.generate_moves();

    moves
        .par_iter()
        .map(|m| {
            let mut new_board = board.clone();
            new_board.apply_move(color, m);
            new_board.promote_kings();
            perft_recur(color.opposite(), &new_board, depth - 1)
        })
        .sum()
}
