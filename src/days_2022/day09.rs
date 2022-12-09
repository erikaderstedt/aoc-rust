// https://adventofcode.com/2022/day/9

use crate::common::Solution;
use std::collections::{HashSet, hash_set};
use std::str::FromStr;
use std::hash::Hash;

#[derive(Eq,PartialEq,Clone,Copy,Hash)]
struct Knot { x: i16, y: i16 }
enum Direction { Up, Down, Left, Right }
struct Instruction { direction: Direction, steps: usize }

const STARTING_POINT: Knot = Knot { x: 0, y: 0};

// This wraps HashSet to remember the last inserted item.
// This will drastically improve perfomance in cases where 
// we often insert the same item as the previously inserted item.
struct LastCheckingHashSet<T: Hash + Eq + Copy> {
    hash_set: HashSet<T>,
    last: T
}

impl<T> LastCheckingHashSet<T> where T: Hash + Eq + Copy {
    fn new(item: T) -> Self {
        Self { hash_set: HashSet::from([item]), last: item }
    }

    fn insert(&mut self, item: T) {
        if self.last != item {
            self.last = item;
            self.hash_set.insert(item);
        }
    } 

    fn len(&self) -> usize { self.hash_set.len() }
}

pub fn solve(input: &str) -> Solution {
    let mut knots = [STARTING_POINT; 10];
    let mut visited_p1 = LastCheckingHashSet::new(STARTING_POINT);
    let mut visited_p2 = LastCheckingHashSet::new(STARTING_POINT);
    
    for line in input.lines() {
        let instruction: Instruction = line.parse().expect("Unable to parse instruction");
        for _ in 0..instruction.steps {
            match instruction.direction {
                Direction::Up => knots[0].y += 1,
                Direction::Down => knots[0].y -= 1,
                Direction::Left => knots[0].x -= 1,
                Direction::Right => knots[0].x += 1,
            };
    
            for i in 1..10 {
                let diff_x = knots[i-1].x - knots[i].x;
                let diff_y = knots[i-1].y - knots[i].y;
                if diff_x.abs() > 1 || diff_y.abs() > 1 {
                    knots[i].x += diff_x.signum();
                    knots[i].y += diff_y.signum();
                }
            }
            visited_p1.insert(knots[2-1]);
            visited_p2.insert(knots[10-1]);
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