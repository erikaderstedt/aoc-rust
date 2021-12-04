// https://adventofcode.com/2015/day/3

use crate::common::Solution;
use std::collections::HashSet;

#[derive(PartialEq,Eq,Hash)]
struct House {
    x: i32,
    y: i32,
}

fn traverse<I>(movements: I) -> HashSet<House> where I: Iterator<Item=char> {
    let mut x = 0;
    let mut y = 0;
    let mut visited_houses = HashSet::new();
    visited_houses.insert(House { x, y });
    for c in movements {
        x = match c { '>' => x + 1, '<' => x - 1, _ => x, };
        y = match c { '^' => y - 1, 'v' => y + 1, _ => y, };
        visited_houses.insert(House { x, y });
    }
    visited_houses
}

pub fn solve(input: &str) -> Solution {
    let m1 = traverse(input.chars()).len();
    let santa = traverse(input.chars().step_by(2));
    let robo_santa = traverse(input.chars().skip(1).step_by(2));
    let m2 = santa.union(&robo_santa).count();

    Solution::new(m1, m2)
}