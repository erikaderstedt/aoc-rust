// https://adventofcode.com/2017/day/5

use crate::common::Solution;

fn execute(instructions: &Vec<i64>, decrease_if_three_or_more: bool) -> usize {
    let mut instructions = instructions.clone();

    let mut ip: usize = 0;
    let mut steps = 0;
    while ip < instructions.len() {
        let v = instructions[ip];

        if decrease_if_three_or_more && v >= 3 {
            instructions[ip] -= 1;
        } else {
            instructions[ip] += 1;
        }

        if v < 0 {
            ip -= v.abs() as usize;
        } else {
            ip += v as usize;
        }
        steps += 1;
    }
    steps
}

pub fn solve(input: &str) -> Solution {
    let instructions: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let p1 = execute(&instructions, false);
    let p2 = execute(&instructions, true);

    Solution::new(p1, p2)
}
