// https://adventofcode.com/2017/day/2

use crate::common::Solution;

fn evenly_divisible_pair(v: &Vec<u64>) -> u64 {
    v.iter().find_map(|value| 
    if let Some(n) = v.iter().find(|o| value != *o && value.rem_euclid(**o) == 0) {
        Some(value / n)
    } else {
        None
    }).unwrap()
}

pub fn solve(input: &str) -> Solution {

    let values: Vec<Vec<u64>> = 
    input.lines().map(|line| line.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect()).collect();
    
    let p1 = values.iter().map(|v| v.iter().max().unwrap() - v.iter().min().unwrap()).sum::<u64>();
    let p2 = values.iter().map(|v| evenly_divisible_pair(v)).sum::<u64>();

    Solution::new(p1,p2)
}
