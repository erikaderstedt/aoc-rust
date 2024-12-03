// https://adventofcode.com/2024/day/3

use crate::common::Solution;
use regex::Regex;

pub fn solve(input: &str) -> Solution {
    let mul = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let sum_in_range = |s: &str| -> usize {
        mul.captures_iter(s)
        .map(|m| m[2].parse::<usize>().unwrap() * m[1].parse::<usize>().unwrap())
        .sum::<usize>()
    };

    let p1 = sum_in_range(input);

    let mut p2 = 0;
    let mut offset = 0;
    loop {
        if let Some(end_here) = input[offset..].find(r"don't()") {
            p2 += sum_in_range(&input[offset..(offset + end_here)]);
            offset += end_here;
        } else {
            p2 += sum_in_range(&input[offset..]);
            break;
        }
        if let Some(start_here) = input[offset..].find(r"do()") {
            offset += start_here;
        } else {
            break;
        }
    }

    Solution::new(p1, p2)
}
