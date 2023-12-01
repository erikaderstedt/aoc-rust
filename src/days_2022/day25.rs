// https://adventofcode.com/2022/day/25

use crate::common::Solution;

fn snafu(input: &str) -> isize {
    input.chars().rev()
        .map(|c| match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Unrecognized SNAFU digit")
        })
        .enumerate()
        .map(|(i, c)| {
            ((5u64.pow(i as u32)) as isize) * c
        })
        .sum()
}

fn to_snafu(i: isize) -> String {
    let mut v = vec![];

    let mut i = i + 2;
    while i > 2 {
        v.push(match i % 5 {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => panic!("")
        });
        i = (i - i % 5) / 5 + 2;
    }

    v.into_iter().rev().collect()
}

pub fn solve(input: &str) -> Solution {
    let n = input.lines().map(|l| snafu(l)).sum();

    let p1 = to_snafu(n);

    Solution::new(p1,0)
}
