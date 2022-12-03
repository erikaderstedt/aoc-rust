// https://adventofcode.com/2022/day/3

use itertools::Itertools;
use crate::common::Solution;

fn priority(c: char) -> usize {
    match c {
        c if c >= 'a' && c <= 'z' => ((c as u8) - ('a' as u8) + 1) as usize,
        c if c >= 'A' && c <= 'Z' => ((c as u8) - ('A' as u8) + 27) as usize,
        _ => 0,
    }
}

pub fn solve(input: &str) -> Solution {
    let p1: usize = input
        .lines()
        .map(|rucksack| {
            rucksack
                .chars()
                .find(|c| rucksack[(rucksack.len() >> 1)..].contains(*c))
                .map(|c| priority(c))
                .unwrap_or(0)
            })
        .sum();

    let p2: usize = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|elf_group_iterator| {
            match elf_group_iterator.collect_tuple() {
                Some((elf1, elf2, elf3)) => 
                    elf1
                        .chars()
                        .find(|c| elf2.contains(*c) && elf3.contains(*c))
                        .map(|c| priority(c))
                        .unwrap_or(0),
                None => 0
            }
        })
        .sum();
    
    Solution::new(p1, p2)
}
