// https://adventofcode.com/2022/day/9

use crate::common::{Solution, parsed_from_each_line};
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

const N: usize = 10;

#[derive(Debug,Eq,PartialEq,Copy,Clone,Hash)]
struct Position {
    x: i32,
    y: i32,
}

enum Direction { Up, Down, Left, Right }

struct Instruction {
    direction: Direction,
    steps: usize,
}

#[derive(Debug)]

struct Rope<const N: usize> {
    knots: [Position;N]
}

impl Rope<N> {
    fn new() -> Rope<N> {
        Rope { knots: [Position { x: 0, y: 0 }; N] }
    }

    fn step(&mut self, direction: &Direction) {
        // Move head
        match direction {
            Direction::Up => self.knots[0].y += 1,
            Direction::Down => self.knots[0].y -= 1,
            Direction::Left => self.knots[0].x -= 1,
            Direction::Right => self.knots[0].x += 1,
        };

        for i in 1..N {
            let diff_x = self.knots[i-1].x - self.knots[i].x;
            let diff_y = self.knots[i-1].y - self.knots[i].y;
            let touching = diff_x.abs() <= 1 && diff_y.abs() <= 1;
            
            if !touching {
                self.knots[i].x += diff_x.signum();
                self.knots[i].y += diff_y.signum();
            }

        }
    }
}

fn follow_instructions<const N: usize>(instructions: &Vec<Instruction>) -> usize {
    let mut rope = Rope::new();
    let mut visited: HashSet<Position> = HashSet::new();

    for instruction in instructions.iter() {
        for _ in 0..instruction.steps {
            rope.step(&instruction.direction);
            visited.insert(rope.knots[N-1]);
        }
    }

    visited.len()
}

pub fn solve(input: &str) -> Solution {
    let instructions: Vec<Instruction> = parsed_from_each_line(input);
    let p1 = follow_instructions::<2>(&instructions);
    let p2 = follow_instructions::<10>(&instructions);

    Solution::new(p1,p2)
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match s.split_once(' ') {            
            Some((dir, n)) => {
                let direction = match dir.as_bytes()[0] as char {
                    'U' => Ok(Direction::Up),
                    'L' => Ok(Direction::Left),
                   'R' => Ok(Direction::Right),
                    'D' => Ok(Direction::Down),
                    _ => Err("Invalid direction value"),
                }?;
                let steps = n.parse::<usize>().map_err(|_| "Invalid num_steps value.")?;
                Ok(Instruction { direction, steps })
            },
            _ => Err("Malformed line."),
        }
    }
}