// https://adventofcode.com/2023/day/1

use itertools::Itertools;
use crate::common::Solution;

pub fn solve(input: &str) -> Solution {    
    let p1: u32 = input
        .split('\n')
        .map(|line| -> u32 {
            let digits: Vec<u32> = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c as u32 - '0' as u32)
                .collect();

            match (digits.first(), digits.last()) {
                (Some(a),Some(b)) => a*10 + b,
                _ => 0,
            }
        })
        .sum();

    let textual_digits = vec!["one","two","three","four","five","six","seven","eight","nine"];
    let p2: usize = input
        .split('\n')
        .map(|line| -> usize {
            let digits: Vec<(usize,usize)> = line
                .chars()
                .enumerate()
                .filter(|c| c.1.is_digit(10))
                .map(|c| (c.0, c.1 as usize - '0' as usize))
                .chain(
                    textual_digits
                        .iter()
                        .enumerate()
                        .map(|(d, t)| line.match_indices(*t).map(move |(idx, _)| (idx, (d+1) as usize)))
                        .flatten()
                )
                .sorted_by(|a,b| Ord::cmp(&a.0, &b.0))
                .collect();

            match (digits.first(), digits.last()) {
                (Some(a),Some(b)) => a.1*10 + b.1,
                _ => 0,
            }
        })
        .sum();

    Solution::new(p1, p2)
}