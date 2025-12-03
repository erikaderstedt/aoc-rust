// https://adventofcode.com/2025/day/3

use crate::common::Solution;

fn best_battery(bank: &Vec<usize>, start_index: usize, remaining: usize) -> usize {
    let n = bank.len();
    let max = bank.iter()
        .skip(start_index)
        .take(n - remaining - start_index)
        .max().unwrap();
    let p = bank.iter()
        .skip(start_index)
        .position(|i| i == max)
        .unwrap() 
        + start_index;

    (if remaining > 0 {
        best_battery(bank, p + 1, remaining - 1) 
    } else { 
        0 
    })
    + max * 10usize.pow(remaining as u32) 
}

pub fn solve(input: &str) -> Solution {
    let banks: Vec<Vec<usize>> = input
        .lines()
        .map(|line| line
            .chars()
            .map(|c| c as u8)
            .map(|c| c - 48)
            .map(|c| c as usize)
            .collect())
        .collect();

    let p1 = banks.iter().map(|bank| best_battery(bank, 0, 1)).sum::<usize>();
    let p2 = banks.iter().map(|bank| best_battery(bank, 0, 11)).sum::<usize>();

    Solution::new(p1, p2)
}
