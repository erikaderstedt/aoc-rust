// https://adventofcode.com/2022/day/2

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let (p1,p2) = input
        .split("\n")
        .flat_map(|line| {
            match line {
                "A X" => Some((1 + 3, 3 + 0)),
                "A Y" => Some((2 + 6, 1 + 3)),
                "A Z" => Some((3 + 0, 2 + 6)),
                "B X" => Some((1 + 0, 1 + 0)),
                "B Y" => Some((2 + 3, 2 + 3)),
                "B Z" => Some((3 + 6, 3 + 6)),
                "C X" => Some((1 + 6, 2 + 0)),
                "C Y" => Some((2 + 0, 3 + 3)),
                "C Z" => Some((3 + 3, 1 + 6)),
                _ => None
            }})
        .fold((0,0), |sum, scores| (sum.0 + scores.0, sum.1 + scores.1));    

    Solution::new(p1, p2)
}
