// https://adventofcode.com/2015/day/20

use crate::common::Solution;

fn distribute_presents(target: usize) -> usize {
    let m = target / 20; // 10, but 20 works and is faster
    let mut presents = vec![0; m];
    for elf in 1..m {
        for house in (elf..m).step_by(elf) {
            presents[house] += elf * 10;
        }
    }
    presents.into_iter().position(|p| p >= target).unwrap()
}

fn distribute_limited_presents(target: usize) -> usize {
    let m = target / 20; // 10, but 20 works and is faster
    let mut presents = vec![0; m];
    for elf in 1..m {
        for house in (elf..m).step_by(elf).take(50) {
            presents[house] += elf * 11;
        }
    }
    presents.into_iter().position(|p| p >= target).unwrap()
}

pub fn solve(input: &str) -> Solution {
    let number_of_presents = input.trim().parse::<usize>().unwrap();

    let p1 = distribute_presents(number_of_presents.clone());
    let p2 = distribute_limited_presents(number_of_presents);

    Solution::new(p1, p2)
}
