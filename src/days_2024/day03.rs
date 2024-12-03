// https://adventofcode.com/2024/day/3

use crate::common::Solution;
use regex::Regex;

pub fn solve(input: &str) -> Solution {
    let mul = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let start = Regex::new(r"do\(\)").unwrap();
    let stop = Regex::new(r"don't\(\)").unwrap();

    let mut started = true;

    let p1 = mul
        .captures_iter(input)
        .map(|m| m[2].parse::<usize>().unwrap() * m[1].parse::<usize>().unwrap())
        .sum::<usize>();

    let mut p2 = 0;
    let mut offset = 0;
    loop {
        if started {
            if let Some(end_here) = stop.find_at(input, offset) {
                p2 += mul
                    .captures_iter(&input[offset..end_here.start()])
                    .map(|m| m[2].parse::<usize>().unwrap() * m[1].parse::<usize>().unwrap())
                    .sum::<usize>();
                offset = end_here.end();
                started = false;
            } else {
                p2 += mul
                    .captures_iter(&input[offset..])
                    .map(|m| m[2].parse::<usize>().unwrap() * m[1].parse::<usize>().unwrap())
                    .sum::<usize>();
                break;
            }
        } else {
            match start.find_at(input, offset) {
                Some(i) => {
                    offset = i.end();
                }
                None => break,
            }
            started = true;
        }
    }

    Solution::new(p1, p2)
}
