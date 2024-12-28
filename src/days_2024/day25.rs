// https://adventofcode.com/2024/day/25

use itertools::Itertools;

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let locks: Vec<[i8;5]> = input
        .split("\n\n")
        .filter_map(|thing| -> Option<[i8;5]> {
            if thing.as_bytes()[0] == b'#' {
                Some([thing.lines().take_while(|line| line.as_bytes()[0] == b'#').count() as i8 - 1,
                thing.lines().take_while(|line| line.as_bytes()[1] == b'#').count() as i8 - 1,
                thing.lines().take_while(|line| line.as_bytes()[2] == b'#').count() as i8 - 1,
                thing.lines().take_while(|line| line.as_bytes()[3] == b'#').count() as i8 - 1,
                thing.lines().take_while(|line| line.as_bytes()[4] == b'#').count() as i8 - 1])
            } else {
                None
            }
        })
        .collect();
    let keys: Vec<[i8;5]> = input
        .split("\n\n")
        .filter_map(|thing| -> Option<[i8;5]> {
            if thing.as_bytes()[0] == b'.' {
                Some([thing.lines().skip_while(|line| line.as_bytes()[0] == b'.').count() as i8 - 1,
                thing.lines().skip_while(|line| line.as_bytes()[1] == b'.').count() as i8 - 1,
                thing.lines().skip_while(|line| line.as_bytes()[2] == b'.').count() as i8 - 1,
                thing.lines().skip_while(|line| line.as_bytes()[3] == b'.').count() as i8 - 1,
                thing.lines().skip_while(|line| line.as_bytes()[4] == b'.').count() as i8 - 1])
            } else {
                None
            }
        })
        .collect();

    let p1 = locks
    .into_iter()
    .cartesian_product(keys)
    .filter(|(lock, key)| (0..5).all(|i| lock[i] + key[i] <= 5))
    .count();

    let p2 = 0;

    Solution::new(p1, p2)
}
