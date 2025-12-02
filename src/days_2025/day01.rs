// https://adventofcode.com/2025/day/1

use crate::common::Solution;

const START: usize = 50;
const SZ: usize = 100;

pub fn solve(input: &str) -> Solution {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut position = START;

    for line in input.lines() {
        let (direction, length) = line.split_at(1);
        let mut length = length.parse::<usize>().unwrap();

        p2 = p2 + length / SZ;
        length = length.rem_euclid(SZ);

        position = match direction {
            "L" => {
                if position <= length && position != 0 { p2 = p2 + 1 };
                (position + SZ - length).rem_euclid(SZ)
            },
            "R" => {
                if position + length >= SZ && position != 0 { p2 = p2 + 1 };
                (position + length).rem_euclid(SZ)
            },
            _ => panic!("invalid direction"),
            };
        
        if position == 0 {
            p1 = p1 + 1;
        }
    }

    Solution::new(p1, p2)
}
