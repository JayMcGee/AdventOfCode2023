use crate::commons::read_lines;
use std::{fmt::Debug, str::FromStr};

struct Game {
    id: i32,
    red_cubes: Vec<i32>,
    green_cubes: Vec<i32>,
    blue_cubes: Vec<i32>,
}
struct ParseGameError;
impl Debug for ParseGameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParseGameError").finish()
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game_str = s
            .strip_prefix("Game ")
            .and_then(|s| s.split_once(':'))
            .ok_or(ParseGameError)?;

        let remains: Vec<&str> = game_str.1.trim().split(";").collect::<Vec<&str>>();

        let mut red: Vec<i32> = Vec::new();
        let mut green: Vec<i32> = Vec::new();
        let mut blue: Vec<i32> = Vec::new();

        // Extract the Rounds from the remains
        for round in remains {
            let draws: Vec<&str> = round.split(',').collect::<Vec<&str>>();

            for draw in draws {
                let tmp = draw.trim().split_once(" ").ok_or(ParseGameError)?;
                match tmp.1 {
                    "red" => red.push(tmp.0.parse::<i32>().unwrap()),
                    "green" => green.push(tmp.0.parse::<i32>().unwrap()),
                    "blue" => blue.push(tmp.0.parse::<i32>().unwrap()),
                    _ => (),
                }
            }
        }

        Ok(Game {
            id: game_str.0.parse::<i32>().unwrap(),
            red_cubes: red,
            green_cubes: green,
            blue_cubes: blue,
        })
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Game")
            .field("id", &self.id)
            .field("red", &self.red_cubes)
            .field("green", &self.green_cubes)
            .field("blue", &self.blue_cubes)
            .finish()
    }
}

fn is_valid(game: Game) -> (bool, i32) {
    // The limits (how many cubes of each colors there are in the bag)
    let r_qty: i32 = 12;
    let g_qty: i32 = 13;
    let b_qty: i32 = 14;

    let mut is_valid = true;

    for red_draws in game.red_cubes {
        if red_draws > r_qty {
            is_valid = false;
        }
    }

    for green_draws in game.green_cubes {
        if green_draws > g_qty {
            is_valid = false;
        }
    }

    for blue_draws in game.blue_cubes {
        if blue_draws > b_qty {
            is_valid = false;
        }
    }

    (is_valid, game.id)
}

fn part2(game: Game) -> i32 {
    // Grab the highest number of each vector
    let min_red = game.red_cubes.iter().fold(std::i32::MIN, |a, b| a.max(*b));
    let min_green = game
        .green_cubes
        .iter()
        .fold(std::i32::MIN, |a, b| a.max(*b));
    let min_blue = game.blue_cubes.iter().fold(std::i32::MIN, |a, b| a.max(*b));

    let power = min_red * min_blue * min_green;
    power
}

pub fn day2() {
    let lines = read_lines("inputs/day2.txt");

    // let mut valid_sum = 0;
    let mut power_sum: i32 = 0;

    for g in lines {
        let game = Game::from_str(&g).unwrap();
        // lets check if the game is valid (Part1)
        // let res = is_valid(game);

        // if res.0 {
        //     valid_sum += res.1;
        //     println!("Game {} is valid sum so far {}", res.1, valid_sum);
        // }

        // Part2
        power_sum += part2(game);
    }

    // Print the final sum
    // println!("Final Part1 sum: {}", valid_sum);
    println!("Final Part2 sum: {}", power_sum);
}
