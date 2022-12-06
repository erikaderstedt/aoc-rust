// https://adventofcode.com/2022/day/6

use crate::common::Solution;

fn check(input: &[u8], window_size: usize) -> usize {
    let mut i = 0;
    loop {
        match (0..(window_size - 1))
                .find(|j| input[(i + j + 1)..(i + window_size)].contains(&input[i + j])) {
            Some(k) => i = i + k + 1,
            None => { return i + window_size; }
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let p1 = check(input.as_bytes(), 4);
    let p2 = check(input.as_bytes(), 14);
    
    Solution::new(p1,p2)
}
