// https://adventofcode.com/2022/day/6

use itertools::Itertools;
use crate::common::Solution;

fn check(input: &str, window_size: usize) -> usize {
    input.as_bytes()
        .windows(window_size)
        .find_position(|w|
            (0..(window_size - 1))
                .find(|i| w[(i+1)..].contains(&w[*i]))
                .is_none()
        )
        .unwrap()
        .0 + window_size
}

pub fn solve(input: &str) -> Solution {
    let p1 = check(input, 4);
    let p2 = check(input, 14);
    
    Solution::new(p1,p2)
}
