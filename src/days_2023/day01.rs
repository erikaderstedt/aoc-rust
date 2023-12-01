// https://adventofcode.com/2022/day/1

use itertools::Itertools;

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {    
    let p1: u32 = input
        .split('\n')
        .map(|line| -> u32 {
            let digits: Vec<u32> = line.chars().filter(|c| c.is_digit(10)).map(|c| c.into()).collect();
            match (digits.first(), digits.last()) {
                (Some(a),Some(b)) => (a - ('0' as u32))*10 + (b - ('0' as u32)),
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
                .filter_map(|(i,c)| if c.is_digit(10) { Some((i, (c as usize) - ('0' as usize)))} else { None })
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