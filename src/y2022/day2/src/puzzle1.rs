use std::collections::HashMap;
use std::fs;
use std::str::Split;

fn calculate_score(game: &str) -> i32 {
    let moves_from_alias: HashMap<&str, &str> = HashMap::from([
        ("A", "ROCK"),
        ("B", "PAPER"),
        ("C", "SCISSORS"),
        ("X", "ROCK"),
        ("Y", "PAPER"),
        ("Z", "SCISSORS"),
    ]);

    let points_for_move = HashMap::from([("ROCK", 1), ("PAPER", 2), ("SCISSORS", 3)]);

    let moves_to_result = HashMap::from([
        (
            "ROCK",
            HashMap::from([("ROCK", 3), ("PAPER", 6), ("SCISSORS", 0)]),
        ),
        (
            "PAPER",
            HashMap::from([("ROCK", 0), ("PAPER", 3), ("SCISSORS", 6)]),
        ),
        (
            "SCISSORS",
            HashMap::from([("ROCK", 6), ("PAPER", 0), ("SCISSORS", 3)]),
        ),
    ]);

    let moves: Vec<&str> = game.split(" ").collect();
    let opponent_move = moves_from_alias[moves[0]];
    let my_move = moves_from_alias[moves[1]];
    return moves_to_result[opponent_move][my_move] + points_for_move[my_move];
}

pub(crate) fn solve() {
    let file_path = "src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let games: Split<&str> = contents.split("\n");

    let mut total_score = 0;

    for game in games {
        if game.is_empty() {
            break;
        }
        let score = calculate_score(game);
        total_score += score;
    }

    println!("Total {}", total_score)
}
