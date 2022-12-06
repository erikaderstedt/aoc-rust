// https://adventofcode.com/2022/day/6

use crate::common::Solution;

fn check(input: &[u8], window_size: usize) -> usize {
    (0..(input.len() - window_size))
        .find(|i|
            (0..(window_size - 1))
                .find(|j| input[(i + j + 1)..(i+window_size)].contains(&input[i + j]))
                .is_none()
        )
        .unwrap() + window_size
}

pub fn solve(input: &str) -> Solution {
    let p1 = check(input.as_bytes(), 4);
    let p2 = check(input.as_bytes(), 14);
    
    Solution::new(p1,p2)
}
