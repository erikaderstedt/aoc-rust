// https://adventofcode.com/2025/day/1

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut position = 50u64;

    for line in input.lines() {
        let (direction, length) = line.split_at(1);
        let length = length.parse::<usize>().unwrap();

        for _ in 0..length {
            position = match direction {
                "L" => (position + 99).rem_euclid(100),
                "R" => (position + 1).rem_euclid(100),
                _ => panic!("wrong direction"),
            };
            if position == 0 {
                p2 = p2 + 1;
            }
        }
        
        if position == 0 {
            p1 = p1 + 1;
        }
    }

    Solution::new(p1, p2)
}
