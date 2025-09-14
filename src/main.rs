use checkers_perft::board::{Board, Color};
use clap::Parser;

/// Run perft for checkers move generation
#[derive(Parser, Debug)]
struct Args {
    /// Depth of the search
    #[arg(short, long)]
    depth: u32,
}

fn main() {
    let args = Args::parse();

    let board = Board::new();
    let color = Color::Black;

    println!(
        "perft({}) = {}",
        args.depth,
        checkers_perft::perft::perft(color, &board, args.depth)
    );
}
