// https://adventofcode.com/2022/day/21

use crate::common::Solution;
use std::collections::HashMap;
use itertools::Itertools;

enum Operation { Add, Subtract, Multiply, Divide, InvDivide }

impl Operation {
    fn evaluate(&self, v1: i64, v2: i64) -> i64 {
        match self {
            Operation::Add => v1 + v2,
            Operation::Divide => v1 / v2,
            Operation::Multiply => v1 * v2,
            Operation::Subtract => v1 - v2,
            Operation::InvDivide => v2 / v1,
        }
    }
}

enum Value<'a> {
    Literal(i64),
    Operation(Operation, &'a str, &'a str),
}

struct Monkeys<'a>(HashMap<&'a str, Value<'a>>);

impl<'a> Monkeys<'a> {
    fn evaluate(&self, name: &str) -> i64 {
        match &self.0[name] {
            Value::Literal(i) => *i,
            Value::Operation(o,a,b) => o.evaluate(self.evaluate(a), self.evaluate(b)),
        }
    }

    fn uses_humn(&self, start: &str) -> bool { 
        match self.0[start] {
            Value::Literal(_) => start == "humn",
            Value::Operation(_,a,b) => self.uses_humn(a) || self.uses_humn(b),
        }
    }

    fn inverse_ops(&self, name: &str) -> Vec<(Operation, i64)> {
        match &self.0[name] {
            Value::Literal(_) => {
                assert!(name == "humn", "Matched a literal but it was not humn");
                return vec![];
            },
            Value::Operation(op, a, b) => {
                let second_operand_uses_humn = self.uses_humn(b);
                let (mut lhs, rhs) = if second_operand_uses_humn { 
                    (self.inverse_ops(b), self.evaluate(a)) 
                } else { 
                    (self.inverse_ops(a), self.evaluate(b)) 
                };
                
                match op {
                    Operation::Add => { lhs.push((Operation::Subtract, rhs)); },
                    Operation::Multiply => { lhs.push((Operation::Divide, rhs)); },
                    Operation::Subtract => { 
                        if second_operand_uses_humn {
                            lhs.push((Operation::Multiply, -1));
                            lhs.push((Operation::Subtract, rhs));
                        } else {
                            lhs.push((Operation::Add, rhs));
                        }
                    },
                    Operation::Divide => {
                        if second_operand_uses_humn {
                            lhs.push((Operation::InvDivide, rhs));
                        } else {
                            lhs.push((Operation::Multiply, rhs));
                        }
                    },
                    Operation::InvDivide => { panic!("The InvDivide operation is not available in the input - it should only be a result of the inverse operations."); }
                }
                lhs                
            }
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let monkeys: Monkeys = input.into();
    let p1 = monkeys.evaluate("root");
    let p2 = match &monkeys.0["root"] {
        Value::Literal(_) => { panic!("Root element cannot be a literal") },
        Value::Operation(_, a, b) => {
            let second_operand_uses_humn = monkeys.uses_humn(b);
            let (mut inverse_operations, mut constant) = if second_operand_uses_humn { 
                (monkeys.inverse_ops(b), monkeys.evaluate(a)) 
            } else { 
                (monkeys.inverse_ops(a), monkeys.evaluate(b)) 
            };
            while let Some((op, value)) = inverse_operations.pop() {
                constant = op.evaluate(constant, value);
            }
            constant
        }
    };

    Solution::new(p1,p2)
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(line: &'a str) -> Self {
        if line.as_bytes()[0].is_ascii_digit() {
            Value::Literal(line.parse().unwrap())
        } else {
            let (monkey1, operator, monkey2) = line.split(" ").collect_tuple().unwrap();
            Value::Operation(match operator {
                "+" => Operation::Add,
                "*" => Operation::Multiply,
                "-" => Operation::Subtract,
                "/" => Operation::Divide,
                _ => { panic!("Unknown operator.") }
            }, monkey1, monkey2)
        }
    }
}

impl<'a> From<&'a str> for Monkeys<'a> {
    fn from(input: &'a str) -> Self {
        Self( input.lines().map(|line| (&line[0..4], line[6..].into() )).collect() )
    }
}