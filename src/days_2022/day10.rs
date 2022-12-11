// https://adventofcode.com/2022/day/10

use crate::common::{Solution, read_5x6_characters_off_grid};
use std::str::FromStr;

enum Command {
    Noop,
    Addx(i32),
}

const NUM_CYCLES: usize = 240;
const ROW_SIZE: i32 = 40;

pub fn solve(input: &str) -> Solution {

    let state: Vec<(i32, usize)> = input
        .lines()
        .map(|line| line.parse::<Command>().expect("Parsing error"))
        .scan(1i32, |x, cmd| match cmd {
                Command::Noop => Some(vec![x.clone()]),
                Command::Addx(v) => {
                    *x += v;
                    Some(vec![*x-v, *x-v])
            }})
        .flatten()
        .zip(1..=NUM_CYCLES)
        .collect();

    let p1: i32 = state
        .iter()
        .skip(20-1)
        .step_by(ROW_SIZE as usize)
        .map(|(x,cycle)| (*cycle as i32) * x)
        .sum();

    let grid: Vec<bool> = state.iter().map(|(x,cycle)| (((*cycle as i32)-1) % ROW_SIZE).abs_diff(*x) <= 1).collect();
    let p2 = read_5x6_characters_off_grid(&grid).expect("Invalid grid size");

    Solution::new(p1,p2)
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..4] {
            "noop" => Ok(Command::Noop),
            "addx" => {
                let n = s[5..].parse::<i32>().map_err(|_| "Invalid addx argument value.")?;
                Ok(Command::Addx(n))
            }
            _ => Err("Unrecognized command")
        }
    }
}