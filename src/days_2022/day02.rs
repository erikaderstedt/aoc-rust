// https://adventofcode.com/2022/day/2

use crate::common::Solution;
use std::str::FromStr;

enum GameResult {
    Win,
    Tie,
    Loss,
}

enum Play {
    Rock,
    Paper,
    Scissors,
}

impl GameResult {
    fn score(&self) -> usize {
        match self {
            GameResult::Win => 6,
            GameResult::Tie => 3,
            GameResult::Loss => 0,
        }
    }
}

impl Play {

    fn value(&self) -> usize {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn score(&self, other: &Play) -> usize {
        let game_result = 
        match (self, other) {
            (Play::Rock, Play::Paper) |
            (Play::Paper, Play::Scissors) |
            (Play::Scissors, Play::Rock) => GameResult::Loss,
            (Play::Rock, Play::Rock) |
            (Play::Paper, Play::Paper) |
            (Play::Scissors, Play::Scissors) => GameResult::Tie,
            (Play::Rock, Play::Scissors) |
            (Play::Paper, Play::Rock) |
            (Play::Scissors, Play::Paper) => GameResult::Win,
        };

        self.value() + game_result.score()
    }
}

pub fn solve(input: &str) -> Solution {
    let p1 = input
        .split("\n")
        .map(|line| {
            match line.split_once(' ') {
                Some((a,b)) => {
                    let other_played = a.parse::<Play>().unwrap();
                    let i_played = b.parse::<Play>().unwrap();        
                    i_played.score(&other_played)
                },
                None => 0
            }})            
        .fold(0, |sum, score| sum + score);
    
    let p2 = input
        .split("\n")
        .map(|line| {
            match line.split_once(' ') {
                Some((a,b)) => {
                    let other_played = a.parse::<Play>().unwrap();
                    let desired_result = b.parse::<GameResult>().unwrap();
                    let i_played = match desired_result {
                        GameResult::Tie => &other_played,
                        GameResult::Loss => match other_played {
                            Play::Rock => &Play::Scissors,
                            Play::Paper => &Play::Rock,
                            Play::Scissors => &Play::Paper,
                        },
                        GameResult::Win => match other_played {
                            Play::Rock => &Play::Paper,
                            Play::Paper => &Play::Scissors,
                            Play::Scissors => &Play::Rock,
                        }};
                        i_played.score(&other_played)
                    },
                None => 0
            }})
        .fold(0, |sum, score| sum + score);

    Solution::new(p1, p2)
}

impl FromStr for Play {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissors),
            _ => Err("Unknown play"),
        }
    }
}

impl FromStr for GameResult {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Loss),
            "Y" => Ok(GameResult::Tie),
            "Z" => Ok(GameResult::Win),
            _ => Err("Unknown game result"),
        }
    }
}