// https://adventofcode.com/2016/day/6

use crate::common::Solution;
use itertools::Itertools;

pub fn solve(input: &str) -> Solution {
    let p1: String = (0..8)
        .map(|i| {
            input
                .lines()
                .map(|line| line.as_bytes()[i])
                .sorted()
                .dedup_with_count()
                .sorted_by_key(|i| i.0)
                .last()
                .unwrap()
                .1 as char
        })
        .collect();

    let p2: String = (0..8)
        .map(|i| {
            input
                .lines()
                .map(|line| line.as_bytes()[i])
                .sorted()
                .dedup_with_count()
                .sorted_by_key(|i| i.0)
                .next()
                .unwrap()
                .1 as char
        })
        .collect();

    Solution::new(p1, p2)
}
