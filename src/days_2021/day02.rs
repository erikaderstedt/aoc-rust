use crate::common::Solution;
use crate::common::parsed_from_each_line;
use std::str::FromStr;

enum Movement {
    Down(i64),
    Up(i64),
    Forward(i64),
}

struct State {
    x: i64,
    depth: i64,
    aim: i64,
}

impl State {
    fn product(&self) -> i64 { self.x * self.depth }
}

impl FromStr for Movement {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let j = s.split(" ").skip(1).next().unwrap().parse::<i64>().map_err(|_| "Invalid integer literal")?;
        match s.split(" ").next().unwrap() {
            "down" => Ok(Movement::Down(j)),
            "up" => Ok(Movement::Up(j)),
            "forward" => Ok(Movement::Forward(j)),
            _ => Err("Bad instruction"),
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let program: Vec<Movement> = parsed_from_each_line(input);

    let part1 = program.iter().fold(State {x: 0, depth: 0, aim: 0}, 
        |p, m| {
        match m {
            Movement::Forward(i) => State { x: p.x + i, depth: p.depth, aim: p.aim },
            Movement::Up(i) => State { x: p.x, depth: p.depth - i, aim: p.aim },
            Movement::Down(i) => State { x: p.x, depth: p.depth + i, aim: p.aim},
        }
    });

    let part2 = program.iter().fold(State {x: 0, depth: 0, aim: 0}, 
        |p, m| {
        match m {
            Movement::Forward(i) => State { x: p.x + i, depth: p.depth + p.aim * i, aim: p.aim },
            Movement::Up(i) => State { x: p.x, depth: p.depth, aim: p.aim - i },
            Movement::Down(i) => State { x: p.x, depth: p.depth, aim: p.aim + i },
        }
    });

    Solution::new(part1.product(), part2.product())
}