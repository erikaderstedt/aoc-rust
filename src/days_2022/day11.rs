// https://adventofcode.com/2022/day/10

use crate::common::{Solution};
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Operation {
    Add(WorryLevel),
    Multiply(WorryLevel),
    Square,
}

impl Operation {
    
    fn apply(&self, worry_level: WorryLevel) -> WorryLevel {
        match self {
            Operation::Add(a) => worry_level + a,
            Operation::Multiply(m) => worry_level * m,
            Operation::Square => worry_level * worry_level,
        }
    }
}

type WorryLevel = usize;
#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<WorryLevel>,
    operation: Operation,
    modulo_test: WorryLevel,
    if_true_throw_to: usize,
    if_false_throw_to: usize,
}

fn simulate<F>(mut monkeys: Vec<Monkey>, op: F, num_rounds: usize) -> usize
        where F: Fn(WorryLevel) -> WorryLevel
{
    let num_monkeys = monkeys.len();
    let mut inspections = vec![0; num_monkeys];
    for _round in 0..num_rounds {
        for i in 0..num_monkeys {
            inspections[i] += monkeys[i].items.len();
            let if_true = monkeys[i].if_true_throw_to;
            let if_false = monkeys[i].if_false_throw_to;
            let modulo_test = monkeys[i].modulo_test;               
        
            while let Some(worry_level) = monkeys[i].items.pop() {
                let worry_level = op(monkeys[i].operation.apply(worry_level));
                let idx = if worry_level % modulo_test == 0 { if_true } else { if_false };
                monkeys[idx].items.push(worry_level);
            }
        }
    }

    inspections.sort();
    inspections[num_monkeys - 1] * inspections[num_monkeys - 2]
}

pub fn solve(input: &str) -> Solution {
    let monkeys: Vec<Monkey> = input.split("\n\n").map(|s| s.parse::<Monkey>().expect("Incorrect monkey")).collect();
    let total_modulo: WorryLevel = monkeys.iter().map(|m| m.modulo_test).product();

    let p1 = simulate(monkeys.clone(), |x| x / 3, 20);
    let p2 = simulate(monkeys, |x| if x > total_modulo { x % total_modulo } else { x }, 10000);

    Solution::new(p1,p2)
}

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').collect();
        let items: Vec<WorryLevel> = lines[1].split_once(": ").unwrap().1
            .split(", ").map(|v| v.parse::<WorryLevel>().unwrap())
            .collect();
        let (op, constant) = lines[2].trim().split(' ').skip(4).collect_tuple()
            .ok_or("Unable to parse operation")?;        
        let operation = match op {
            "+" => Ok(Operation::Add(constant.parse::<WorryLevel>().unwrap())),
            "*" if constant == "old" => Ok(Operation::Square),
            "*" => Ok(Operation::Multiply(constant.parse::<WorryLevel>().unwrap())),
            _ => Err("Unrecognized operation"),
        }?;
        let modulo_test = lines[3].trim().split(' ').skip(3).next().unwrap().parse::<WorryLevel>().map_err(|_| "Invalid modulo value")?;
        let if_true_throw_to = lines[4].trim().split(' ').skip(5).next().unwrap().parse::<usize>().map_err(|_| "Invalid throw destination if true")?;
        let if_false_throw_to = lines[5].trim().split(' ').skip(5).next().unwrap().parse::<usize>().map_err(|_| "Invalid throw destination if false")?;
        Ok(Monkey {
            items,
            operation,
            modulo_test,
            if_true_throw_to,
            if_false_throw_to,
        })
    }
}
