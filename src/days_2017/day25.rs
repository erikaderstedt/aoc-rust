// https://adventofcode.com/2017/day/25

use crate::common::Solution;
use itertools::Itertools;
use std::str::FromStr;

enum Direction {
    Left,
    Right,
}

struct Action {
    value_to_write: u8,
    direction: Direction,
    next_state: usize,
}

struct State {
    when_zero: Action,
    when_one: Action,
}

pub fn solve(input: &str) -> Solution {
    let states: Vec<State> = input
        .split("\n\n")
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let start = match input.lines().next().unwrap().split(' ').last().unwrap() {
        "A." => 0,
        "B." => 1,
        "C." => 2,
        "D." => 3,
        "E." => 4,
        "F." => 5,
        _ => panic!("unknown state"),
    };
    let checksum_after_step: usize = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .find_map(|s| s.parse::<usize>().ok())
        .unwrap();

    let mut memory = vec![0u8; 1_000_000];
    let offset = memory.len() / 2;
    let mut state_index = start;
    let mut mem_index = offset;
    for _ in 0..checksum_after_step {
        let s = &states[state_index];
        let action = if memory[mem_index] == 1 {
            &s.when_one
        } else {
            &s.when_zero
        };
        memory[mem_index] = action.value_to_write;
        match action.direction {
            Direction::Left => mem_index -= 1,
            Direction::Right => mem_index += 1,
        }
        state_index = action.next_state;
    }

    let p1 = memory.into_iter().filter(|s| *s == 1).count();
    let p2 = 0;

    Solution::new(p1, p2)
}

impl Action {
    fn parse(lines: &[&str]) -> Action {
        let (value, direction, next_state) = lines
            .iter()
            .map(|line| {
                let s = line.split(' ').last().unwrap();
                s[0..(s.len() - 1)].to_string()
            })
            .collect_tuple()
            .unwrap();
        Action {
            value_to_write: value.parse::<u8>().unwrap(),
            direction: if direction == "left" {
                Direction::Left
            } else {
                Direction::Right
            },
            next_state: match next_state.as_str() {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                "D" => 3,
                "E" => 4,
                "F" => 5,
                _ => panic!("Invalid state"),
            },
        }
    }
}

impl FromStr for State {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().skip(1).collect();
        let when_zero = Action::parse(&lines[1..4]);
        let when_one = Action::parse(&lines[5..8]);
        Ok(State {
            when_zero,
            when_one,
        })
    }
}
