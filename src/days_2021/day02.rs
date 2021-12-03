// https://adventofcode.com/2021/day/2

use crate::common::Solution;
use crate::common::parsed_from_each_line;
use std::str::FromStr;

enum Movement {
    Down(i64),
    Up(i64),
    Forward(i64),
}

impl FromStr for Movement {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((command, magnitude)) => {
                let x = magnitude.parse::<i64>().map_err(|_| "Invalid integer literal")?;
                match command {
                    "down" => Ok(Movement::Down(x)),
                    "up" => Ok(Movement::Up(x)),
                    "forward" => Ok(Movement::Forward(x)),
                    _ => Err("Bad instruction"),
                }},
            _ => Err("Malformed line."),
        }
    }
}

struct State {
    x: i64,
    depth: i64,
    aim: i64,
}

impl State {
    fn product(&self) -> i64 { self.x * self.depth }
}

pub fn solve(input: &str) -> Solution {
    let program: Vec<Movement> = parsed_from_each_line(input);

    let part1 = program.iter().fold( State {x: 0, depth: 0, aim: 0}, 
        |current, movement| {
        match movement {
            Movement::Forward(i) => State { x: current.x + i, depth: current.depth, aim: current.aim },
            Movement::Up(i) => State { x: current.x, depth: current.depth - i, aim: current.aim },
            Movement::Down(i) => State { x: current.x, depth: current.depth + i, aim: current.aim},
        }
    });

    let part2 = program.iter().fold( State {x: 0, depth: 0, aim: 0}, 
        |current, movement| {
        match movement {
            Movement::Forward(i) => State { x: current.x + i, depth: current.depth + current.aim * i, aim: current.aim },
            Movement::Up(i) => State { x: current.x, depth: current.depth, aim: current.aim - i },
            Movement::Down(i) => State { x: current.x, depth: current.depth, aim: current.aim + i },
        }
    });

    Solution::new(part1.product(), part2.product())
}