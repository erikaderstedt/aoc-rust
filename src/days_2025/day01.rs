// https://adventofcode.com/2025/day/1

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut position = 50;

    for line in input.lines() {
        let (direction, length) = line.split_at(1);
        let mut length = length.parse::<usize>().unwrap();

        while length >= 100 {
            length = length - 100;
            p2 = p2 + 1;
        }

        position = match direction {
            "L" => { 
                if position <= length && position != 0 { p2 = p2 + 1 };
                (position + 100 - length).rem_euclid(100)
            },
            "R" => {
                if position + length > 99 && position != 0 { p2 = p2 + 1 };
                (position + length).rem_euclid(100)
            },
            _ => panic!("wrong direction"),
            };
        
        if position == 0 {
            p1 = p1 + 1;
        }
    }

    Solution::new(p1, p2)
}
