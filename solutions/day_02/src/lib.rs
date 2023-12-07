pub mod game {
    #[derive(PartialEq, Debug)]
    pub struct GameDetails (pub u16, pub Vec<GameState>);
    #[derive(PartialEq, Debug)]
    pub struct GameState {
        pub blue: u16,
        pub red: u16,
        pub green: u16
    }
}

use game::{GameDetails, GameState};


pub fn game_string_to_vec(game_string: String) -> Vec<String>  {
    let split: Vec<String> = game_string.split(":").map(str::to_string).collect();
    split
}

pub fn parse_game_number(game_number: &String) -> u16 {
    game_number.split_whitespace().last().unwrap_or("0").parse().unwrap()
}

fn parse_game_state(game: &str)  -> GameState {
    let color_counts: Vec<Vec<&str>> = game.split(",").map(|color_count| color_count.split_whitespace().collect::<Vec<&str>>()).collect();

    let mut game_state = GameState {blue: 0, red: 0, green: 0};

    for color_count in color_counts {
        match color_count.get(1) {
            Some(&color) => {
                match color {
                    "red" => game_state.red = color_count[0].parse().unwrap(),
                    "green" => game_state.green = color_count[0].parse().unwrap(),
                    "blue" => game_state.blue = color_count[0].parse().unwrap(),
                    _ => todo!()
                }
            },
            None => todo!(),
        }
    }
    game_state
}

fn parse_game_states_string(games_string: &String) -> Vec<GameState>{
    games_string.split(";").map(|game| {
        parse_game_state(game)
    }).collect::<Vec<GameState>>()
}

fn parse_game_details(game_string_vec: Vec<String>) -> GameDetails {
    let game_number: u16 = parse_game_number(&game_string_vec[0]);
    let game_states: Vec<GameState> = parse_game_states_string(&game_string_vec[1]);

    GameDetails(game_number, game_states)
}

pub fn deserialize_game(game_string: String) -> GameDetails {
    let vec_game_partition = game_string_to_vec(game_string);
    parse_game_details(vec_game_partition)
}

pub fn is_game_possible(game_details: &GameDetails, red: u16, green: u16, blue: u16) -> bool {
    for game in &game_details.1 {
        if game.blue > blue {
            return false
        }
        if game.red >  red {
            return false
        }
        if game.green > green {
            return false
        }
    }
    true
}