// https://adventofcode.com/2015/day/5

use crate::common::Solution;
use crate::common::parsed_from_each_line;
use itertools::Itertools;
use std::str::FromStr;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug,PartialEq)]
enum Operation {
    And,
    Or,
    Lshift,
    Rshift,
    Not,
}
impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Operation::And),
            "OR" => Ok(Operation::Or),
            "LSHIFT" => Ok(Operation::Lshift),
            "RSHIFT" => Ok(Operation::Rshift),
            "NOT" => Ok(Operation::Not),
            _ => Err("Unknown operation")
        }
    }
}
#[derive(Debug,PartialEq)]
enum Operand {
    Literal(u16),
    Wire(String),
}

impl FromStr for Operand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(v) => Ok(Operand::Literal(v)),
            Err(_) => Ok(Operand::Wire(s.to_string())),
        }
    }
}
#[derive(Debug,PartialEq)]
struct Connection {
    operation: Operation,
    left: Option<Operand>,
    right: Operand,
}

impl Connection {
    fn evaluate(&self) -> Option<u16> {
        match self.right {
            Operand::Literal(r) => match self.left {
                Some(Operand::Literal(l)) => Some(match self.operation {
                    Operation::And => l & r,
                    Operation::Or => l | r,
                    Operation::Lshift => l << r,
                    Operation::Rshift => l >> r,
                    Operation::Not => panic!("NOT operation should not have a left operand."),
                }),
                Some(Operand::Wire(_)) => None,
                None => Some(match self.operation {
                    Operation::Not => !r,
                    _ => panic!("Operation {:?} should have a left operand", self.operation),
                })},
            _ => None,
        }
    }
}
#[derive(Debug,PartialEq)]
enum Value {
    Literal(u16),
    Connection(Connection),
}


pub fn solve(input: &str) -> Solution {
    // Deal with literals
    let mut instructions: HashMap<String, Value> = HashMap::new();
    let r = Regex::new("([a-z0-9]+)? ?(AND|NOT|OR|RSHIFT|LSHIFT)? ?([a-z0-9]+) -> ([a-z]+)").unwrap();

    for line in input.lines() {

        match r.captures(line) {
            None => panic!("Unable to match line {}", line),
            Some(c) => {
                let w = c.get(4).unwrap().as_str().to_string();
                instructions.insert(w, match c.get(2) {
                    None => Value::Literal(c.get(3).unwrap().as_str().parse::<u16>().unwrap()), 
                    Some(m) => Value::Connection( {
                        let operation = m.as_str().parse::<Operation>().unwrap();
                        let right = c.get(3).unwrap().as_str().parse::<Operand>().unwrap();
                        if operation == Operation::Not {
                            Connection { operation, left: None, right}
                        } else {
                            let left = Some(c.get(1).unwrap().as_str().parse::<Operand>().unwrap());
                            Connection { operation, left, right}
                    }}),
                });
            }
        }

    }
    // loop through instructions where the values are 
    for (wire_name, value) in instructions.iter_mut() {
        match value {
            Value::Connection(c) => {
                if c.evaluate()
            }
        }
    }.filter(|(_, v)| match v { Value::Connection(_) => true, _ => false}) {

    }
println!("{:?}", instructions);
    Solution::new(0, 0)
}