// https://adventofcode.com/2016/day/3

use crate::common::Solution;
use itertools::Itertools;

fn possible(s1: u16, s2: u16, s3: u16) -> u32 {
    if s1 + s2 > s3 && s1 + s3 > s2 && s2 + s3 > s1 {
        1
    } else {
        0
    }
}

pub fn solve(input: &str) -> Solution {
    let values: Vec<(u16, u16, u16)> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<u16>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let p1: u32 = values.iter().map(|(a, b, c)| possible(*a, *b, *c)).sum();
    let p2: u32 = values
        .chunks(3)
        .map(|a| {
            possible(a[0].0, a[1].0, a[2].0)
                + possible(a[0].1, a[1].1, a[2].1)
                + possible(a[0].2, a[1].2, a[2].2)
        })
        .sum();

    Solution::new(p1, p2)
}
