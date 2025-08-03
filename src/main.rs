use checkers_perft::board::{Board, Color};

fn main() {
    let board = Board::new();
    let color = Color::Black;

    // println!("perft(0)  = {}", checkers_perft::perft::perft(color, &board, 0));
    // println!("perft(1)  = {}", checkers_perft::perft::perft(color, &board, 1));
    // println!("perft(2)  = {}", checkers_perft::perft::perft(color, &board, 2));
    // println!("perft(3)  = {}", checkers_perft::perft::perft(color, &board, 3));
    // println!("perft(4)  = {}", checkers_perft::perft::perft(color, &board, 4));
    // println!("perft(5)  = {}", checkers_perft::perft::perft(color, &board, 5));
    // println!("perft(6)  = {}", checkers_perft::perft::perft(color, &board, 6));
    // println!("perft(7)  = {}", checkers_perft::perft::perft(color, &board, 7));
    // println!("perft(8)  = {}", checkers_perft::perft::perft(color, &board, 8));
    // println!("perft(9)  = {}", checkers_perft::perft::perft(color, &board, 9));
    // println!("perft(10) = {}", checkers_perft::perft::perft(color, &board, 10));
    // println!("perft(11) = {}", checkers_perft::perft::perft(color, &board, 11));
    println!("perft(12) = {}", checkers_perft::perft::perft(color, &board, 12));
}
