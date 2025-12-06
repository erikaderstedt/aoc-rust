// https://adventofcode.com/2025/day/6

use crate::common::Solution;

enum Operator {
    Multiply,
    Add
}

struct Block {
    operator: Operator,
    start: usize,
    end: usize
}

pub fn solve(input: &str) -> Solution {
    let n = input.lines().count() - 1;
    let operator_line = input.lines().last().unwrap();
    let blocks: Vec<Block> = operator_line.chars()
        .enumerate()
        .filter_map(|(i,c)| match c {
            '*' => Some(Block::from(Operator::Multiply, i, operator_line)),
            '+' => Some(Block::from(Operator::Add, i, operator_line)),        
            _ => None
        })
        .collect();
    
    let p1: usize = blocks.iter().map(|block| {
        input.lines()
            .take(n)
            .map(|line| line[block.start..block.end].trim().parse::<usize>().unwrap())
            .fold(block.operator.identity(), |acc, value| block.operator.apply(acc, value))
    }).sum::<usize>();

    let p2: usize = blocks.iter().map(|block| {
        (block.start..block.end).map(|column| {
            input.lines()
                .take(n)
                .flat_map(|line| line[column..(column+1)].parse::<usize>().ok())
                .fold(0, |acc, digit| 10*acc + digit)
        })
        .fold(block.operator.identity(), |acc, value| block.operator.apply(acc, value))
    })
    .sum::<usize>();

    Solution::new(p1, p2)
}

impl Block {
    fn from(operator: Operator, start: usize, line: &str) -> Block {
        let end = match line[(start+1)..].find(|c| c == '*' || c == '+') {
            Some(n) => start + n,
            None => line.len()
        };
        Block { operator, start, end }
    }
}

impl Operator {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b
        }
    }

    fn identity(&self) -> usize {
        match self {
            Operator::Add => 0,
            Operator::Multiply => 1
        }
    }
}
