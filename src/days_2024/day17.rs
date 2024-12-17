// https://adventofcode.com/2024/day/17

use std::collections::HashSet;
use itertools::Itertools;
use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let (register_s, program_s) = input.split_once("\n\n").unwrap();
    let (mut a, mut b, mut c) = register_s.lines().map(|line| line.split(": ").last().unwrap().parse::<isize>().unwrap()).collect_tuple().unwrap();
    let program: Vec<usize> = program_s.split(": ").last().unwrap().trim().split(",").map(|s| s.parse::<usize>().unwrap()).collect();

    let mut ip = 0;

    let mut v = vec![];
    loop {
        let to_combo = |o: usize| -> isize {
            match o {
                0..= 3 => o as isize,
                4 => a,
                5 => b,
                6 => c,
                _ => panic!("Invalid combo operand"),
            }
        };

        if ip >= program.len() - 1 {
            break;
        }
        let instruction = program[ip];
        let operand = program[ip + 1];
        match instruction {
            0 => { a = a >> to_combo(operand); ip += 2; },
            1 => { b = b ^ (operand as isize); ip += 2; },
            2 => { b = to_combo(operand) & 7; ip += 2; },
            3 => { if a > 0 { ip = operand; } else { ip += 2; } },
            4 => { b = b ^ c; ip += 2; },
            5 => { v.push(to_combo(operand) & 7); ip += 2; },
            6 => { b = a >> to_combo(operand); ip += 2; },
            7 => { c = a >> to_combo(operand); ip += 2; },
            _ => panic!("Unknown instruction!"),
        }
    }
    let p1 = v.into_iter().join(",");

    // a is a 48 bit number
    // 2,4, 1,5,        b = ((a % 8) ^ 5)
    // 7,5,             c = a >> ((a % 8) ^ 5)        (moved up here)
    // 1,6              b = b ^ 6 = (((a % 8) ^ 5) ^6) = (a % 8) ^ 3
    // 4,0,             b = b ^ c = ((a % 8) ^ 3) ^ (a >> ((a % 8) ^ 5))
    // 5,5,             output lowest 3 bits of b
    // 0,3,             a = a >> 3
    // 3,0              loop if a != 0
    // the output is a function of a in each iteration
    // b and c are not kept

    let mut possible_top_parts: HashSet<usize> = HashSet::new();
    possible_top_parts.insert(0);
    for i in program.iter().rev() {
        possible_top_parts = possible_top_parts
            .into_iter()
            .map(|e: usize| {
                (0..8).filter_map(move |bit| -> Option<usize> {
                    let a = (e << 3) + bit;
                    if ((((a % 8) ^ 3) ^ (a >> ((a % 8) ^ 5))) & 7) == *i {
                        Some(a)
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect();
    }
    let p2 = possible_top_parts.iter().min().unwrap();

    Solution::new(p1, p2)
}

