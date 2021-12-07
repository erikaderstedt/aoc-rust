// https://adventofcode.com/2021/day/6
use crate::common::Solution;

fn num_lanternfish(mut state: [usize;9], num_days: usize) -> usize {
    for _ in 0..num_days {
        let breeders = state[0];
        for i in 1..9 {
            state[i-1] = state[i];
        }
        state[6] += breeders;
        state[8] = breeders;
    }
    state.iter().sum()
}

pub fn solve(input: &str) -> Solution {
    let mut num_of_different_ages = [0usize; 9];
    for num in input.split(',').map(|s| s.parse::<usize>().unwrap()) {
        num_of_different_ages[num] += 1;
    }

    let m1 = num_lanternfish(num_of_different_ages.clone(), 80);
    let m2 = num_lanternfish(num_of_different_ages, 256);
    Solution::new(m1, m2)
}
