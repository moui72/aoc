use std::collections::HashMap;
use std::fs;
use std::str::Split;

fn calculate_score(game: &str) -> i32 {
    let moves_from_alias: HashMap<&str, &str> =
        HashMap::from([("A", "ROCK"), ("B", "PAPER"), ("C", "SCISSORS")]);

    let points_for_result = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);

    let points_for_move = HashMap::from([("ROCK", 1), ("PAPER", 2), ("SCISSORS", 3)]);

    let move_to_result_to_move = HashMap::from([
        (
            "ROCK",
            HashMap::from([(3, "ROCK"), (6, "PAPER"), (0, "SCISSORS")]),
        ),
        (
            "PAPER",
            HashMap::from([(0, "ROCK"), (3, "PAPER"), (6, "SCISSORS")]),
        ),
        (
            "SCISSORS",
            HashMap::from([(6, "ROCK"), (0, "PAPER"), (3, "SCISSORS")]),
        ),
    ]);

    let moves: Vec<&str> = game.split(" ").collect();
    let opponent_move = moves_from_alias[moves[0]];
    let desired_result = points_for_result[moves[1]];
    let my_move = move_to_result_to_move[opponent_move][&desired_result];
    return desired_result + points_for_move[my_move];
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
