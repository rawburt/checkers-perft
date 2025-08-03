use checkers_perft::{
    board::{Board, Color},
    game_json::Game,
    move_generator::MoveGenerator,
};

#[cfg(test)]
fn test_game(game: Game) {
    let mut board = Board::new();
    for m in game.moves {
        let generated_moves = MoveGenerator::new(board.clone(), Color::Black).generate_moves();
        let black_move = m.black.into_move();
        assert!(
            generated_moves.contains(&black_move),
            "number= {}\nheaders= {}\nblack_move= {}\ngenerated_moves= {:?}\nboard= {}",
            m.number,
            game.headers
                .iter()
                .map(|h| format!("{}: {}", h.name, h.value))
                .collect::<Vec<_>>()
                .join(", "),
            black_move.to_string(),
            generated_moves
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>(),
            board.to_string()
        );
        board.apply_move(Color::Black, &black_move);
        board.promote_kings();
        if let Some(white_move_detail) = m.white {
            let white_move = white_move_detail.into_move();
            let generated_moves = MoveGenerator::new(board.clone(), Color::White).generate_moves();
            assert!(
                generated_moves.contains(&white_move),
                "number= {}\nheaders= {}\nwhite_move= {}\ngenerated_moves= {:?}\nboard= {}",
                m.number,
                game.headers
                    .iter()
                    .map(|h| format!("{}: {}", h.name, h.value))
                    .collect::<Vec<_>>()
                    .join(", "),
                white_move.to_string(),
                generated_moves
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>(),
                board.to_string()
            );
            board.apply_move(Color::White, &white_move);
            board.promote_kings();
        }
    }
}

#[test]
fn test_game1() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/game1.json");
    let game = Game::from_file(path).expect("Failed to parse game JSON");
    test_game(game);
}

#[test]
fn test_game2() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/game2.json");
    let game = Game::from_file(path).expect("Failed to parse game JSON");
    test_game(game);
}

#[test]
fn test_alwick() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/alwick.json");
    let game = Game::from_file(path).expect("Failed to parse game JSON");
    test_game(game);
}

#[test]
fn test_edin8() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/edin8.json");
    let game = Game::from_file(path).expect("Failed to parse game JSON");
    test_game(game);
}

// #[test]
// fn test_oca() {
//     let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/oca.json");
//     let games = parse_games_from_json_array_file(path).expect("Failed to parse game JSON array");
//     for game in games {
//         test_game(game);
//     }
// }
