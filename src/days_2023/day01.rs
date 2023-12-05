// https://adventofcode.com/2023/day/1

use crate::common::Solution;

fn calibration_values(input: &str) -> u32 {
    input
    .split('\n')
    .map(|line|
        match ( line.chars().find(|c| c.is_ascii_digit()), 
                line.chars().rev().find(|c| c.is_ascii_digit())) {
            (Some(first), Some(last)) => (first as u32 - '0' as u32) * 10 + last as u32 - '0' as u32,
            _ => 0,
        })
    .sum()
}

pub fn solve(input: &str) -> Solution {    
    let p1 = calibration_values(input);

    let modified = input
    .replace("one","o1e")
    .replace("two", "t2o")
    .replace("three", "th3ee")
    .replace("four", "fo4r")
    .replace("five", "fi5e")
    .replace("six", "s6x")
    .replace("seven", "se7en")
    .replace("eight", "ei8ht")
    .replace("nine", "ni9e");

    let p2: u32 = calibration_values(&modified);

    Solution::new(p1, p2)
}