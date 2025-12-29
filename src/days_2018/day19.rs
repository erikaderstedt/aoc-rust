// https://adventofcode.com/2018/day/19

use crate::common::Solution;
use itertools::Itertools;

enum Operation {
    Addi,
    Addr,
    Eqrr,
    Gtrr,
    Muli,
    Mulr,
    Seti,
    Setr,
}

struct Instruction {
    op: Operation,
    a: usize,
    b: usize,
    out: usize,
}

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
        let i = &instructions[ip];

        match i.op {
            Operation::Addi => registers[i.out] = registers[i.a] + i.b,
            Operation::Addr => registers[i.out] = registers[i.a] + registers[i.b],
            Operation::Muli => registers[i.out] = registers[i.a] * i.b,
            Operation::Mulr => registers[i.out] = registers[i.a] * registers[i.b],
            Operation::Eqrr => {
                registers[i.out] = if registers[i.a] == registers[i.b] {
                    1
                } else {
                    0
                }
            }
            Operation::Gtrr => {
                registers[i.out] = if registers[i.a] > registers[i.b] {
                    1
                } else {
                    0
                }
            }
            Operation::Seti => registers[i.out] = i.a,
            Operation::Setr => registers[i.out] = registers[i.a],
        }
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
    let (first_line, rest) = input.split_once("\n").unwrap();
    let ip_register = first_line
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .next()
        .unwrap();

    let instructions: Vec<Instruction> = rest
        .lines()
        .map(|s| {
            let (op, a, b, out) = s.split(' ').collect_tuple().unwrap();
            let op = match op {
                "addi" => Operation::Addi,
                "addr" => Operation::Addr,
                "muli" => Operation::Muli,
                "mulr" => Operation::Mulr,
                "seti" => Operation::Seti,
                "setr" => Operation::Setr,
                "gtrr" => Operation::Gtrr,
                "eqrr" => Operation::Eqrr,
                _ => panic!("Unknown instruction"),
            };
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            let out = out.parse::<usize>().unwrap();
            Instruction { op, a, b, out }
        })
        .collect();

    let p1 = run_program(&instructions, 0, ip_register);
    let p2 = run_program(&instructions, 1, ip_register);
    Solution::new(p1, p2)
}
