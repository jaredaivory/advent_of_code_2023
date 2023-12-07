use std::io::Error;
use std::io::prelude::BufRead;
use utils::file::read_file;
use adventofcode_2023_day_02::{deserialize_game, game::GameDetails};


struct MinimumCubes {
    red: u16,
    green: u16,
    blue: u16,
}

fn get_minimum_needed_cubes(game: GameDetails) -> MinimumCubes {
    let mut min = MinimumCubes {
        red: 0,
        green: 0,
        blue: 0
    };
    for game in game.1 {
        min.red = std::cmp::max(min.red, game.red);
        min.green = std::cmp::max(min.green, game.green);
        min.blue = std::cmp::max(min.blue, game.blue);
    }
    min
}


pub fn start() -> Result<(), Error> {
    let reader = read_file("solutions/day_02/input.txt").expect("Err");
    let mut game_sum = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let game_details = deserialize_game(line.to_string());
                let minimum_cubes = get_minimum_needed_cubes(game_details);
                game_sum += minimum_cubes.red * minimum_cubes.green * minimum_cubes.blue
            },
            Err(error) => panic!("Problem opening the file: {:?}", error)
        }
    }

    println!("\n\n\nAdvent Of Code - 2023 | Day 02 | 'Cube Conundrum': Part 2 \n Answer: {}", game_sum);
    Ok(())
}

#[cfg(test)]
mod tests{

    use test_case::test_case;
    use adventofcode_2023_day_02::game::{GameDetails, GameState};
    use adventofcode_2023_day_02::{deserialize_game, is_game_possible};
    use  super::*;

    #[test_case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" => 48)]
    #[test_case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue" => 12)]
    #[test_case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red" => 1560)]
    #[test_case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red" => 630)]
    #[test_case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green" => 36)]
    fn get_minimum_cubes_power(s: &str) -> u16 {
        let game_details = deserialize_game(s.to_string());
        let minimum_cubes = get_minimum_needed_cubes(game_details);
        minimum_cubes.red * minimum_cubes.green * minimum_cubes.blue
    }


    #[test_case(
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green" =>  2286 )]
    fn test_game_output(s: &str) -> u16 {
        let mut result: u16 = 0;
        for line in s.to_string().lines() {
            let game_details = deserialize_game(line.to_string());
            let minimum_cubes = get_minimum_needed_cubes(game_details);
            result += minimum_cubes.red * minimum_cubes.green * minimum_cubes.blue
        }
        result
    }
}