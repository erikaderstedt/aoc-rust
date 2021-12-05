// https://adventofcode.com/2015/day/6

use crate::common::Solution;
use crate::common::parsed_from_each_line;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = if s.starts_with("toggle") {
            Action::Toggle
        } else if s.starts_with("turn on") {
            Action::TurnOn
        } else {
            Action::TurnOff
        };
        let (r2, r1): (&str,&str) = s.split(' ').rev().step_by(2).take(2).collect_tuple().unwrap();
        let (min_x, min_y): (usize, usize) = r1.split(',').map(|q| q.parse::<usize>().unwrap()).collect_tuple().unwrap();
        let (max_x, max_y): (usize, usize) = r2.split(',').map(|q| q.parse::<usize>().unwrap()).collect_tuple().unwrap();
        Ok(Instruction { action, min_x, min_y, max_x, max_y})
    }
}

pub fn solve(input: &str) -> Solution {
    let instructions: Vec<Instruction> = parsed_from_each_line(input);

    let m1 = {
        let mut grid = [[0u32;1000];1000];
        for instruction in instructions.iter() {
            match instruction.action {
                Action::TurnOn => 
                    for y in instruction.min_y..=instruction.max_y { 
                        for x in instruction.min_x..=instruction.max_x {
                            grid[y][x] = 1;
                        }
                    },
                Action::TurnOff => 
                    for y in instruction.min_y..=instruction.max_y { 
                        for x in instruction.min_x..=instruction.max_x {
                            grid[y][x] = 0;
                        }
                    },            
                Action::Toggle => 
                    for y in instruction.min_y..=instruction.max_y { 
                        for x in instruction.min_x..=instruction.max_x {
                            grid[y][x] = 1-grid[y][x];
                        }
                    },                        
            }
        }
        let mut m1 = 0;
        for y in 0..1000 {
            for x in 0..1000 {
                if grid[y][x] > 0 {
                    m1 += 1;
                }
            }
        }
        m1
    };

    let m2 = {
        let mut grid = [[0u32;1000];1000];
        for instruction in instructions.iter() {
            match instruction.action {
                Action::TurnOn => 
                    for y in instruction.min_y..=instruction.max_y { 
                        for x in instruction.min_x..=instruction.max_x {
                            grid[y][x] += 1;
                        }
                    },
                Action::TurnOff => 
                    for y in instruction.min_y..=instruction.max_y { 
                        for x in instruction.min_x..=instruction.max_x {
                            if grid[y][x] > 0 { grid[y][x] -= 1 };
                        }
                    },            
                Action::Toggle => 
                    for y in instruction.min_y..=instruction.max_y { 
                        for x in instruction.min_x..=instruction.max_x {
                            grid[y][x] += 2;
                        }
                    },                        
            }
        }
        let mut m2 = 0;
        for y in 0..1000 {
            m2 += grid[y][..].iter().sum::<u32>();
        }
        m2
    };

    Solution::new(m1, m2)
}
