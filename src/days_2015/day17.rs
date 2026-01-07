// https://adventofcode.com/2015/day/17

use crate::common::Solution;
use itertools::Itertools;

const TARGET: i64 = 150;

pub fn solve(input: &str) -> Solution {
    let containers: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let p1 = (0..containers.len())
        .map(|k| {
            containers
                .iter()
                .combinations(k)
                .filter(|c| c.iter().cloned().sum::<i64>() == TARGET)
                .count()
        })
        .sum::<usize>();
    let p2 = (0..containers.len())
        .find_map(|k| {
            let num = containers
                .iter()
                .combinations(k)
                .filter(|c| c.iter().cloned().sum::<i64>() == TARGET)
                .count();
            if num > 0 {
                Some(num)
            } else {
                None
            }
        })
        .unwrap();

    Solution::new(p1, p2)
}
