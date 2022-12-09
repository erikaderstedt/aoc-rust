// https://adventofcode.com/2022/day/9

use crate::common::Solution;
use std::str::FromStr;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Eq,PartialEq,Clone,Copy,Hash)]
struct KnotPosition { x: i16, y: i16 }
enum Direction { Up, Down, Left, Right }
struct Instruction { direction: Direction, steps: usize }

const STARTING_POINT: KnotPosition = KnotPosition { x: 0, y: 0};

pub fn solve(input: &str) -> Solution {
    let mut knots = [STARTING_POINT; 10];
    let mut visited_p1: HashSet<KnotPosition> = HashSet::new();
    let mut visited_p2: HashSet<KnotPosition> = HashSet::new();
    visited_p1.insert(STARTING_POINT);
    visited_p2.insert(STARTING_POINT);
    
    for line in input.lines() {
        let instruction: Instruction = line.parse().expect("Unable to parse instruction");
        for _ in 0..instruction.steps {
            match instruction.direction {
                Direction::Up => knots[0].y += 1,
                Direction::Down => knots[0].y -= 1,
                Direction::Left => knots[0].x -= 1,
                Direction::Right => knots[0].x += 1,
            };
            if let Some(num_iterations) = (1..10)
                .take_while(|&i| {
                    let diff_x = knots[i-1].x - knots[i].x;
                    let diff_y = knots[i-1].y - knots[i].y;
                    if diff_x.abs() > 1 || diff_y.abs() > 1 {
                        knots[i].x += diff_x.signum();
                        knots[i].y += diff_y.signum();
                        true
                    } else { false }})
                .last() {
                visited_p1.insert(knots[2-1]);
                if num_iterations == 9 {
                    visited_p2.insert(knots[10-1]);
                }
            }
        }
    }
    
    Solution::new(visited_p1.len(), visited_p2.len())
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