use regex::Regex;

mod read;
mod lib;

use lib::{MarbleGame};

fn main() {

    let game_parameters_string = read::read_lines("input.txt".to_owned()).into_iter().nth(0).unwrap();

    let parameters_regex : Regex = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let capture_groups_regex = parameters_regex.captures(&game_parameters_string).unwrap();
    let number_players : usize = capture_groups_regex.get(1).unwrap().as_str().parse().unwrap();
    let last_marble_value : usize = capture_groups_regex.get(2).unwrap().as_str().parse().unwrap();
    
    
    // Part 1
    let marble_game : MarbleGame = MarbleGame::new(number_players, last_marble_value);
    let scores_map = marble_game.play_game();
    let max_score : (usize, i64) = scores_map.into_iter()
        .map(|(player, scores)| (player, scores.into_iter().sum()))
        .max_by_key(|(_, score)| *score)
        .unwrap();
    println!("\r🔮  Max score of '{}' achieved by player '{}' (Part 1)", max_score.1, max_score.0);
    
    // Part 2
    let marble_game : MarbleGame = MarbleGame::new(number_players, last_marble_value * 100);
    let scores_map = marble_game.play_game();
    let max_score : (usize, i64) = scores_map.into_iter()
        .map(|(player, scores)| (player, scores.into_iter().sum()))
        .max_by_key(|(_, score)| *score)
        .unwrap();
    println!("\r🔮  Max score of '{}' achieved by player '{}' (Part 2)", max_score.1, max_score.0);
}