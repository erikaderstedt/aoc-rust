// https://adventofcode.com/2021/day/7
use crate::common::Solution;

fn energy_cost(crabs: &Vec<usize>, destination: usize) -> usize {
    crabs.iter().fold(0usize, |s, c| s + 
        if *c < destination { 
            destination - *c 
        } else { 
            *c - destination
        })
}

fn energy_cost2(crabs: &Vec<usize>, destination: usize) -> usize {
    crabs.iter().fold(0usize, |s, c| s + 
        if *c < destination { 
            (destination - *c)*(destination - *c + 1)/2 
        } else { 
            (*c - destination)*(*c - destination + 1)/2
        })
}

pub fn solve(input: &str) -> Solution {
    let numbers: Vec<usize> = input.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

    let x = numbers.len();
    let m1 = (0..x).map(|i| energy_cost(&numbers, i)).min().unwrap();
    let m2 = (0..x).map(|i| energy_cost2(&numbers, i)).min().unwrap();

    Solution::new(m1, m2)
}
