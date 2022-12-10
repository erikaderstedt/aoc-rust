// https://adventofcode.com/2022/day/10

use crate::common::{Solution, read_5x6_character_off_grid};
use std::str::{FromStr, from_utf8};

enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    fn num_cycles(&self) -> i32 {
        match self {
            Command::Noop => 1,
            Command::Addx(_) => 2,
        }
    }

    fn modify_register(&self, x: &mut i32) {
        match self {
            Command::Addx(v) => *x += v,
            Command::Noop => {},
        }
    }   
}

const NUM_CYCLES: usize = 240;
const NUM_CHARS: usize = 8;

fn read_character(grid: &[bool;NUM_CYCLES], index: usize) -> char {
    let mut a = [false; 30];
    for i in 0..30 { a[i] = grid[(i/5) * 5 * NUM_CHARS + index*5 + (i % 5)]; }

    read_5x6_character_off_grid(&a)
}

pub fn solve(input: &str) -> Solution {
    let cmds = input.lines().map(|line| line.parse::<Command>().unwrap());

    let mut grid = [false; NUM_CYCLES];    
    let mut cycle: i32 = 0;
    let mut x: i32 = 1;
    let mut p1: i32 = 0;

    for cmd in cmds {
        let cycles_in_command = cmd.num_cycles();
        for i in 0..cycles_in_command {
            if (cycle % 40).abs_diff(x) <= 1 {
                grid[cycle as usize] = true;
            }

            cycle += 1; 
            if (cycle - 20) % 40 == 0 {
                p1 += cycle * x;
            }
            if i == cycles_in_command - 1 {
                cmd.modify_register(&mut x);
            }
        }

        if cycle > 240 { break; }
    }
    
    let p2: Vec<u8> = (0..8).map(|i| read_character(&grid, i) as u8).collect();

    Solution::new(p1,from_utf8(&p2[..]).unwrap())
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {            
            Some((d, n)) if d == "addx" => {
                let n = n.parse::<i32>().map_err(|_| "Invalid addx argument value.")?;
                Ok(Command::Addx(n)) },
            None if s == "noop" => Ok(Command::Noop),
            _ => Err("Malformed line."),
        }
    }
}