// https://adventofcode.com/2022/day/10

use crate::common::{Solution, read_5x6_character_off_grid};
use std::str::{FromStr, from_utf8};

enum Command {
    Noop,
    Addx(i32),
}

const NUM_CYCLES: usize = 240;
const NUM_CHARS: usize = 8;

fn read_character(grid: &[bool;NUM_CYCLES], index: usize) -> char {
    let mut a = [false; 30];
    for i in 0..30 { a[i] = grid[(i/5) * 5 * NUM_CHARS + index*5 + (i % 5)]; }

    read_5x6_character_off_grid(&a)
}

pub fn solve(input: &str) -> Solution {
    let (_, p1, grid) = input
        .lines()
        .map(|line| line.parse::<Command>().expect("Parsing error"))
        .flat_map(|cmd| match cmd {
            Command::Noop => vec![Command::Noop].into_iter(),
            Command::Addx(v) => { vec![Command::Noop, Command::Addx(v)].into_iter() }
        })
        .zip(1..=NUM_CYCLES)
        .fold((1i32, 0i32, [false; NUM_CYCLES]),
            |(x, mut p1_acc, mut grid), (cmd, cycle)| {
                if (((cycle - 1) as i32) % 40).abs_diff(x) <= 1 {
                    grid[cycle-1] = true;
                }
                if (cycle + 20) % 40 == 0 {
                    p1_acc += (cycle as i32) * x;
                }
                match cmd {
                    Command::Addx(v) => (x + v, p1_acc, grid),
                    Command::Noop => (x, p1_acc, grid),
                }
        });
    let p2: Vec<u8> = (0..8).map(|i| read_character(&grid, i) as u8).collect();

    Solution::new(p1,from_utf8(&p2[..]).unwrap())
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