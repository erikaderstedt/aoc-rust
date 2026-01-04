// https://adventofcode.com/2017/day/6

use itertools::Itertools;

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let mut banks: [u8; 16] = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u8>().unwrap())
        .collect_array()
        .unwrap();

    let mut seen: Vec<[u8; 16]> = vec![];

    let mut p1 = 0;
    loop {
        let mut redistribute = banks.len() - 1 - banks.iter().rev().position_max().unwrap();
        let mut how_many = banks[redistribute];
        banks[redistribute] = 0;
        while how_many > 0 {
            redistribute += 1;
            if redistribute == banks.len() {
                redistribute = 0;
            }
            banks[redistribute] += 1;
            how_many -= 1;
        }
        p1 += 1;
        if seen.contains(&banks) {
            break;
        }
        seen.push(banks.clone());
    }
    let p2 = seen.len() - seen.iter().position(|p| *p == banks).unwrap();

    Solution::new(p1, p2)
}
