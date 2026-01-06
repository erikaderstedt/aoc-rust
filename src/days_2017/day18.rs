// https://adventofcode.com/2017/day/18

use crate::common::{parsed_from_each_line, Solution};
use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, Clone)]
enum Operand {
    Register(usize),
    Immediate(i64),
}
type Register = usize;

#[derive(Debug, Clone)]
enum Operation {
    Set(Register, Operand),
    Mul(Register, Operand),
    Add(Register, Operand),
    Mod(Register, Operand),
    Jgz(Operand, Operand),
    Rcv(Register),
    Snd(Register),
}

#[derive(Debug, Clone)]
struct Program {
    operations: Vec<Operation>,
    registers: [i64; 26],
    ip: i64,
    queue: VecDeque<i64>,
    terminated: bool,
}

impl Program {
    fn operand(&self, operand: Operand) -> i64 {
        match operand {
            Operand::Immediate(x) => x,
            Operand::Register(u) => self.registers[u],
        }
    }

    fn run(&mut self) -> VecDeque<i64> {
        let mut emitted_frequencies: VecDeque<i64> = VecDeque::new();

        while !self.terminated && self.ip >= 0 && self.ip < (self.operations.len() as i64) {
            let operation = self.operations[self.ip as usize].clone();

            match operation {
                Operation::Add(register, o) => self.registers[register] += self.operand(o),
                Operation::Mul(register, o) => self.registers[register] *= self.operand(o),
                Operation::Mod(register, o) => self.registers[register] %= self.operand(o),
                Operation::Set(register, o) => self.registers[register] = self.operand(o),
                Operation::Jgz(o1, o2) => {
                    if self.operand(o1) > 0 {
                        self.ip += self.operand(o2);
                        continue;
                    }
                }
                Operation::Rcv(register) => {
                    if let Some(v) = self.queue.pop_front() {
                        self.registers[register] = v;
                    } else {
                        return emitted_frequencies;
                    }
                }
                Operation::Snd(register) => emitted_frequencies.push_back(self.registers[register]),
            };
            self.ip += 1;
        }
        self.terminated = true;
        emitted_frequencies
    }
}

pub fn solve(input: &str) -> Solution {
    let operations = parsed_from_each_line(input);

    let mut program_1 = Program {
        operations: operations.clone(),
        registers: [0; 26],
        ip: 0,
        terminated: false,
        queue: VecDeque::new(),
    };

    let mut program_2 = program_1.clone();
    program_2.registers[(('p' as u8) - ('a' as u8)) as usize] = 1;

    program_2.queue = program_1.run();

    let p1 = program_2.queue.back().cloned().unwrap();

    let mut p2 = 0;
    while !program_1.terminated && !program_2.terminated {
        p2 -= program_1.queue.len();
        program_1.queue.extend(program_2.run());
        p2 += program_1.queue.len();

        if program_1.queue.len() == 0 && program_2.queue.len() == 0 {
            break;
        }

        program_2.queue.extend(program_1.run());
    }

    Solution::new(p1, p2)
}

impl FromStr for Operand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(value) => Ok(Operand::Immediate(value)),
            _ => Ok(Operand::Register((s.as_bytes()[0] - ('a' as u8)) as usize)),
        }
    }
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        match parts[0] {
            "add" | "mul" | "mod" | "set" => {
                let register = (parts[1].as_bytes()[0] - ('a' as u8)) as usize;
                let op = parts[2].parse()?;
                match parts[0] {
                    "add" => Ok(Operation::Add(register, op)),
                    "mul" => Ok(Operation::Mul(register, op)),
                    "mod" => Ok(Operation::Mod(register, op)),
                    "set" => Ok(Operation::Set(register, op)),
                    _ => panic!("?"),
                }
            }
            "rcv" | "snd" => {
                let register = (parts[1].as_bytes()[0] - ('a' as u8)) as usize;
                match parts[0] {
                    "rcv" => Ok(Operation::Rcv(register)),
                    "snd" => Ok(Operation::Snd(register)),
                    _ => panic!("?"),
                }
            }
            "jgz" => {
                let op1 = parts[1].parse()?;
                let op2 = parts[2].parse()?;
                Ok(Operation::Jgz(op1, op2))
            }
            _ => Err("Unknown operation"),
        }
    }
}
