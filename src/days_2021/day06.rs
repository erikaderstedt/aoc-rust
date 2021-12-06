// https://adventofcode.com/2021/day/6
use crate::common::Solution;

fn num_lanternfish(state: &mut [usize;9], num_days: usize) -> usize {
    for _ in 0..num_days {
        let spawns = state[0];
        for i in 1..9 {
            state[i-1] = state[i];
        }
        state[6] += spawns;
        state[8] = spawns;
    }
    state[0] + state[1] + state[2] +
    state[3] + state[4] + state[5] +
    state[6] + state[7] + state[8]
}

pub fn solve(input: &str) -> Solution {

    let mut num_of_different_ages = [0usize; 9];
    for num in input.split(',').map(|s| s.parse::<usize>().unwrap()) {
        num_of_different_ages[num] += 1;
    }

    let m1 = num_lanternfish(&mut num_of_different_ages.clone(), 80);
    let m2 = num_lanternfish(&mut num_of_different_ages.clone(), 256);
    Solution::new(m1, m2)
}