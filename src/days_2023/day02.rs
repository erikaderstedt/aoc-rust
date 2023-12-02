// https://adventofcode.com/2023/day/2

use std::str::FromStr;
use crate::common::{Solution, parsed_from_each_line};

struct Attempt {
    blue: usize,
    green: usize,
    red: usize,
}

impl Attempt {
    fn possible(&self, num_red: usize, num_green: usize, num_blue: usize) -> bool {
        self.blue <= num_blue && self.green <= num_green && self.red <= num_red
    }
}

struct Game {
    id: usize,
    attempts: Vec<Attempt>
}

impl Game {
    fn possible(&self, num_red: usize, num_green: usize, num_blue: usize) -> bool {
        self.attempts.iter().all(|a| a.possible(num_red, num_green, num_blue))
    }

    fn power_of_fewest_possible_cubes(&self) -> usize {
        let required_red = self.attempts.iter().map(|a| a.red).max().unwrap_or(0);
        let required_green = self.attempts.iter().map(|a| a.green).max().unwrap_or(0);
        let required_blue = self.attempts.iter().map(|a| a.blue).max().unwrap_or(0);
        
        required_blue * required_green * required_red
    }
}

pub fn solve(input: &str) -> Solution {    
    let games: Vec<Game> = parsed_from_each_line(input);

    let pt1: usize = games.iter()
        .filter(|game| game.possible(12, 13, 14))
        .map(|game| game.id)
        .sum();

    let pt2: usize = games.iter()
        .map(|game| game.power_of_fewest_possible_cubes())
        .sum();

    Solution::new(pt1,pt2)
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.split(":").next().unwrap().split(" ").last().unwrap().parse::<usize>().map_err(|_| "Invalid game number.")?;
        let attempts = s.split(":").last().unwrap().split(";").map(|q| q.parse::<Attempt>().expect("Unable to parse attempt.")).collect();
        Ok(Game { id, attempts })
    }
}

impl FromStr for Attempt {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nfind = |color_name: &str| -> usize {
            if let Some(i) = s.find(color_name) {
                s[..(i-1)].split(" ").last().unwrap()
                .parse::<usize>().ok().unwrap_or(0)
            } else {
                0
            }            
        };
        Ok(Attempt { blue: nfind("blue"), green: nfind("green"), red: nfind("red") })
    }
}