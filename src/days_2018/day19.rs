// https://adventofcode.com/2018/day/19

use crate::{common::Solution, days_2018::operation::Instruction};

fn run_program(
    instructions: &Vec<Instruction>,
    initial_register_value: usize,
    ip_register: usize,
) -> usize {
    let mut registers = [0usize; 6];
    registers[0] = initial_register_value;
    let mut ip = 0;
    for _ in 0..100 {
        // This is enough to get the target number into a register.
        registers[ip_register] = ip;
        instructions[ip].execute(&mut registers);

        ip = registers[ip_register] + 1;
    }
    // The program is trying to find the sum of all factors of a number (not just all prime factors)
    // This number will be the largest number among the registers, once the program has had time to set it up
    // (typically 20 or so instructions, but we use 100 to make sure)
    // We get the sum of factors with the following code (not very efficient, but vastly more efficient than
    // the original program).
    let target = registers.iter().max().unwrap().clone();
    let mut result = 1;
    for v in 2..=target {
        if target.rem_euclid(v) == 0 {
            result += v;
        }
    }
    result
}

pub fn solve(input: &str) -> Solution {
    let (ip_register, instructions) = Instruction::parse_program(input);

    let p1 = run_program(&instructions, 0, ip_register);
    let p2 = run_program(&instructions, 1, ip_register);
    Solution::new(p1, p2)
}
