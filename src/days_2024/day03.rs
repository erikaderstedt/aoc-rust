// https://adventofcode.com/2024/day/3

use crate::common::Solution;
use regex::Regex;

pub fn solve(input: &str) -> Solution {
    let mul = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let stop_start = Regex::new(r"(?s)don't\(\).*?(do\(\)|$)").unwrap();

    let sum_in_range = |s: &str| -> usize {
        mul.captures_iter(s)
        .map(|m| m[2].parse::<usize>().unwrap() * m[1].parse::<usize>().unwrap())
        .sum::<usize>()
    };

    let p1 = sum_in_range(input);
    let p2 = sum_in_range(&stop_start.replace_all(&input, " "));

    Solution::new(p1, p2)
}
