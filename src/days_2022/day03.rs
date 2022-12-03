// https://adventofcode.com/2022/day/2

use itertools::Itertools;
use crate::common::Solution;

fn priority(c: Option<char>) -> usize {
    match c {
        Some(c) if c >= 'a' && c <= 'z' => ((c as u8) - ('a' as u8) + 1) as usize,
        Some(c) if c >= 'A' && c <= 'Z' => ((c as u8) - ('A' as u8) + 27) as usize,
        _ => 0,
    }
}

pub fn solve(input: &str) -> Solution {
    let p1: usize = input
        .lines()
        .map(|line| {
            let w = line.len();
            let wh = w >> 1;
            priority(line
                .chars()
                .find(|c| line[wh..w].contains(*c)))
            })
        .sum();

    let p2: usize = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|elf_group_iterator| {
            if let Some((elf1, elf2, elf3)) = elf_group_iterator.collect_tuple() {
                priority(elf1
                    .chars()
                    .find(|c| elf2.contains(*c) && elf3.contains(*c)))
            } else { 
                0
            }
        })
        .sum();
    
    Solution::new(p1, p2)
}
