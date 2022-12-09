// https://adventofcode.com/2022/day/9

use crate::common::Solution;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug,Eq,PartialEq,Copy,Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {

    fn touching(&self, other: &Position) -> bool {
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;
        diff_x.abs() <= 1 && diff_y.abs() <= 1
    }

}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug)]

struct State {
    head: Position,
    tail: Position,
}

impl State {
    fn new() -> State {
        State { head: Position { x: 0, y: 0 }, tail: Position { x: 0, y: 0}}
    }

    fn step(&mut self, direction: &str) {
        // Move head
        match direction {
            "U" => self.head.y += 1,
            "D" => self.head.y -= 1,
            "L" => self.head.x -= 1,
            "R" => self.head.x += 1,
            _ => println!("WARNING: unrecognized direction '{}'.", direction),
        };


        if !self.head.touching(&self.tail) {
            let diff_x = self.head.x - self.tail.x;
            let diff_y = self.head.y - self.tail.y;
            if diff_y > 1 {
                self.tail.y +=1;
                self.tail.x = self.head.x;
            } else if diff_y < -1 {
                self.tail.y -=1;
                self.tail.x = self.head.x;
            } else if diff_x > 1 {
                self.tail.x += 1;                    
                self.tail.y = self.head.y;
            } else if diff_x < -1 {
                    self.tail.x -= 1;                    
                self.tail.y = self.head.y;
            }
        }

        println!("Move: '{}': State {:?}", direction, self);

    } 
    
}

pub fn solve(input: &str) -> Solution {

    let mut state = State::new();
    let mut visited: HashSet<Position> = HashSet::new();

    for line in input.lines() {
        if let Some((direction, num_steps)) = line.split_once(" ") {
            let num_steps = num_steps.parse::<usize>().unwrap();
            for _ in 0..num_steps {
                state.step(direction);
                visited.insert(state.tail);
            }
        }
    }

    let p1 = visited.len();

    Solution::new(p1,0)
}

