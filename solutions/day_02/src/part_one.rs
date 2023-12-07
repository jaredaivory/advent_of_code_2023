use std::io::Error;
use std::io::prelude::BufRead;
use utils::file::read_file;

use adventofcode_2023_day_02::deserialize_game;
use adventofcode_2023_day_02::is_game_possible;


pub fn start() -> Result<(), Error> {
    let reader = read_file("solutions/day_02/input.txt").expect("Err");
    let mut game_sum = 0;

    let red = 12;
    let green = 13;
    let blue = 14;


    for line in reader.lines() {
        match line {
            Ok(line) => {
                let game = deserialize_game(line.to_string());
                if is_game_possible(&game, red, green, blue){
                    game_sum += game.0;
                }
            },
            Err(error) => panic!("Problem opening the file: {:?}", error)
        }
    }

    println!("\n\n\nAdvent Of Code - 2023 | Day 02 | 'Cube Conundrum': Part 1 \n Answer: {}", game_sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use adventofcode_2023_day_02::game::{GameDetails, GameState};
    use adventofcode_2023_day_02::{game_string_to_vec, is_game_possible, parse_game_number};

    use super::*;

    #[test_case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" => GameDetails(1, vec![
        GameState {blue: 3, red: 4, green: 0},
        GameState {blue: 6, red: 1, green: 2},
        GameState {blue: 0, red: 0, green: 2}
    ]))]
    #[test_case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue" => GameDetails(2, vec![
        GameState {blue: 1, red: 0, green: 2},
        GameState {blue: 4, red: 1, green: 3},
        GameState {blue: 1, red: 0, green: 1}
    ]))]
    #[test_case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red" => GameDetails(3, vec![
        GameState {blue: 6, red: 20, green: 8},
        GameState {blue: 5, red: 4, green: 13},
        GameState {blue: 0, red: 1, green: 5}
    ]))]
    #[test_case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red" => GameDetails(4, vec![
        GameState {blue: 6, red: 3, green: 1},
        GameState {blue: 0, red: 6, green: 3},
        GameState {blue: 15, red: 14, green: 3}
    ]))]
    #[test_case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green" => GameDetails(5, vec![
        GameState {blue: 1, red: 6, green: 3},
        GameState {blue: 2, red: 1, green: 2}
    ]))]
    fn create_game_states(s: &str) -> GameDetails {
        deserialize_game(s.to_string())
    }


    #[test_case(
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 12, 13, 14 =>  8)]
    fn test_game_output(s: &str, red: u16, green: u16,  blue: u16) -> u16 {
        let mut result: u16 = 0;
        for line in s.to_string().lines() {
            let game = deserialize_game(line.to_string());
            if is_game_possible(&game, red, green, blue){
                result += game.0;
            }
        }
        result
    }


    #[test_case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";"split game string")]
    fn test_gamestring_to_vec(game_string: &str){
        let game_vec = game_string_to_vec(game_string.to_string());
        assert_eq!(game_vec.len(), 2)
    }

    #[test_case("Game 1" => 1; "parse game number 1")]
    #[test_case("Game 20" => 20;"parse game number 20")]
    fn test_parse_game_number(game_string: &str) -> u16 {
        let game_number: u16  = parse_game_number(&game_string.to_string());
        println!("{}", game_number);
        game_number
    }
}