// https://adventofcode.com/2022/day/1

use crate::common::Solution;
use itertools::Itertools;

pub fn solve(input: &str) -> Solution {
    let elves: Vec<usize> = input
        .split("\n\n")
        .map(|a| a
            .split("\n")
            .map(|x| x.parse::<usize>().unwrap_or(0))
            .fold(0, |sum, val| sum + val))
        .sorted()
        .collect();

    let p1 = elves[elves.len() - 1];
    let p2 = p1 + elves[elves.len() - 2] + elves[elves.len() - 3];

    Solution::new(p1, p2)
}