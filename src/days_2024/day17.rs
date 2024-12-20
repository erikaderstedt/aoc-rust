// https://adventofcode.com/2024/day/17

use std::collections::HashSet;
use itertools::Itertools;
use crate::common::Solution;

struct Executor<'a> {
    a: isize,
    b: isize,
    c: isize,
    program: &'a Vec<usize>,
    ip: usize,
}

impl<'a> Executor<'a> {
    fn run(a: isize, program: &'a Vec<usize>) -> Executor<'a> {
        Executor { a, b: 0, c: 0, program, ip: 0}
    }
}

impl<'a> Iterator for Executor<'a> {
    type Item=isize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let to_combo = |o: usize| -> isize {
                match o {
                    0..= 3 => o as isize,
                    4 => self.a,
                    5 => self.b,
                    6 => self.c,
                    _ => panic!("Invalid combo operand"),
                }
            };
    
            if self.ip >= self.program.len() - 1 {
                break None;
            }
            let instruction = self.program[self.ip];
            let operand = self.program[self.ip + 1];
            match instruction {
                0 => { self.a = self.a >> to_combo(operand); self.ip += 2; },
                1 => { self.b = self.b ^ (operand as isize); self.ip += 2; },
                2 => { self.b = to_combo(operand) & 7; self.ip += 2; },
                3 => { if self.a > 0 { self.ip = operand; } else { self.ip += 2; } },
                4 => { self.b = self.b ^ self.c; self.ip += 2; },
                5 => { let v = to_combo(operand) & 7; self.ip += 2; break Some(v); },
                6 => { self.b = self.a >> to_combo(operand); self.ip += 2; },
                7 => { self.c = self.a >> to_combo(operand); self.ip += 2; },
                _ => panic!("Unknown instruction!"),
            }
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let (register_s, program_s) = input.split_once("\n\n").unwrap();
    let (a, b, c) = register_s.lines().map(|line| line.split(": ").last().unwrap().parse::<isize>().unwrap()).collect_tuple().unwrap();
    let program: Vec<usize> = program_s.split(": ").last().unwrap().trim().split(",").map(|s| s.parse::<usize>().unwrap()).collect();

    let executor = Executor { a, b, c, program: &program, ip: 0};

    let p1 = executor.map(|x| x.to_string()).join(",");

    // The code below is general enough to work as long as
    // 1) b and c are not carried over between outputs (and start at 0)
    // 2) a is reduced by three bits after each output

    let mut possible_top_parts: HashSet<usize> = HashSet::new();
    possible_top_parts.insert(0);
    for i in program.clone().iter().rev() {
        possible_top_parts = possible_top_parts
            .into_iter()
            .map(|e: usize| {
                let possibilities: Vec<usize> = 
                (0..8).filter_map(|bit| -> Option<usize> {
                    let a = ((e << 3) + bit) as isize;
                    if Executor::run(a, &program).next() == Some(*i as isize) {
                        Some(a as usize)
                    } else {
                        None
                    }
                }).collect();
                possibilities.into_iter()
            })
            .flatten()
            .collect();
    }
    let p2 = possible_top_parts.iter().min().unwrap();

    Solution::new(p1, p2)
}

