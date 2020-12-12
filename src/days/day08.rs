use crate::common::Solution;
use crate::common::parsed_from_each_line;
use std::str::FromStr;

#[derive(Debug,Clone)]
enum Instruction { Jmp(i64), Acc(i64), Nop(i64), }

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let j = s[4..].parse::<i64>().map_err(|_| "Invalid integer literal")?;
        match &s[0..3] {
            "acc" => Ok(Instruction::Acc(j)),
            "jmp" => Ok(Instruction::Jmp(j)),
            "nop" => Ok(Instruction::Nop(j)),
            _ => Err("Bad instruction"),
        }
    }
}

enum ExitMode { RevisitedInstruction, EndOfProgram, }

fn run_program(program: &Vec<Instruction>) -> (ExitMode, i64) {
    let mut i = 0;
    let mut visited = vec![false;program.len()];
    let mut accumulator: i64 = 0;

    loop {
        if visited[i] { return (ExitMode::RevisitedInstruction, accumulator) }
        visited[i] = true;
        match program[i] {
            Instruction::Acc(x) => { accumulator += x; i += 1 },
            Instruction::Nop(_) => { i += 1 },
            Instruction::Jmp(x) => { i = ((i as i64) + x) as usize },
        }
        if i >= program.len() { return (ExitMode::EndOfProgram, accumulator) }
    }
}

pub fn solve(input: &str) -> Solution {
    let mut program: Vec<Instruction> = parsed_from_each_line(input);

    let p1 = run_program(&program).1;
    let p2: i64 = (0..program.len()).find_map(|index| {
            let old = program[index].clone();
            program[index] = match program[index] {
                Instruction::Acc(_) => { return None },
                Instruction::Jmp(x) => Instruction::Nop(x),
                Instruction::Nop(x) => Instruction::Jmp(x),
            };
            let accumulator_value = match run_program(&program) {
                (ExitMode::RevisitedInstruction, _) => None,
                (ExitMode::EndOfProgram, i) => Some(i)
            };
            program[index] = old;
            accumulator_value
        }).unwrap();

    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}