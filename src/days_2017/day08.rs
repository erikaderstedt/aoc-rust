// https://adventofcode.com/2017/day/8

use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::common::{Solution, parsed_from_each_line};

enum Operation {
    Increase,
    Decrease,
}

impl Operation {
    fn evaluate(&self, value1: i64, value2: i64) -> i64 {
        match self {
            Operation::Increase => value1 + value2,
            Operation::Decrease => value1 - value2,
        }
    }
}

enum Comparison {
    LessThan,
    LessThanOrEqual,
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
    NotEqual,
}

impl Comparison {
    fn evaluate(&self, value1: i64, value2: i64) -> bool {
        match self {
            Comparison::LessThan => value1 < value2,
            Comparison::LessThanOrEqual => value1 <= value2,
            Comparison::GreaterThan => value1 > value2,
            Comparison::GreaterThanOrEqual => value1 >= value2,
            Comparison::Equal => value1 == value2,
            Comparison::NotEqual => value1 != value2,
        }
    }
}

type Register = u32;

struct Line {
    operand1: Register,
    operation: Operation,
    operand2: i64,
    conditional_operand_1: Register,
    comparison: Comparison,
    conditional_operand_2: i64,
}

pub fn solve(input: &str) -> Solution {
    let lines: Vec<Line> = parsed_from_each_line(input);

    let mut registers: HashMap<u32, i64> = HashMap::new();

    let mut p1 = 0;
    let mut p2 = 0;
    for line in lines.iter() {
        let value1 = registers
            .get(&line.conditional_operand_1)
            .unwrap_or(&0);
        if line.comparison.evaluate(*value1, line.conditional_operand_2) {
            if let Some(r) = registers.get_mut(&line.operand1) {
                *r = line.operation.evaluate(*r, line.operand2)
            } else {
                registers.insert(line.operand1, line.operation.evaluate(0, line.operand2));
            }
        }
        p1 = registers.values().max().unwrap().clone();
        if p2 < p1 { p2 = p1; }
    }

    Solution::new(p1,p2)
}

impl FromStr for Comparison {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Self::GreaterThan),
            ">=" => Ok(Self::GreaterThanOrEqual),
            "<" => Ok(Self::LessThan),
            "<=" => Ok(Self::LessThanOrEqual),
            "==" => Ok(Self::Equal),
            "!=" => Ok(Self::NotEqual),
            _ => Err("Unknown comparison operation")
        }
    }
    
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(Self::Increase),
            "dec" => Ok(Self::Decrease),
            _ => Err("Unknwon increase/decrease operation")
        }
    }
    
}

fn register_from_str(s: &str) -> Register {
    s.as_bytes().iter().fold(0, |acc, v| {
        (acc << 8) + (*v as u32)
    })
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (reg, op, op2, _, reg2, cmp, op3) = s.split(' ').collect_tuple().unwrap();
        let comparison = cmp.parse()?;
        let operation = op.parse()?;
        let operand2 = op2.parse().map_err(|_| "Invalid operand 2")?;
        let conditional_operand_2 = op3.parse().map_err(|_| "Invalid operand 3")?;
        let operand1 = register_from_str(reg);
        let conditional_operand_1 = register_from_str(reg2);
        Ok(Self { operand1, operation, operand2, conditional_operand_1, comparison, conditional_operand_2 })
    }
    
}