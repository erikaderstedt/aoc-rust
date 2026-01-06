// https://adventofcode.com/2015/day/7

use crate::common::Solution;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    And,
    Or,
    Lshift,
    Rshift,
    Not,
    Set,
}

#[derive(Debug, PartialEq)]
enum Operand {
    Literal(u16),
    Wire(String),
}

#[derive(Debug, PartialEq)]
struct Connection {
    operation: Operation,
    left: Option<Operand>,
    right: Operand,
}

impl Operand {
    fn value(&self, wires: &HashMap<String, u16>) -> Option<u16> {
        match self {
            Self::Literal(x) => Some(x.clone()),
            Self::Wire(s) => wires.get(s).cloned(),
        }
    }
}

impl Connection {
    fn prerequisites_ok(&self, wires: &HashMap<String, u16>) -> bool {
        self.right.value(wires).is_some()
            && if let Some(left) = &self.left {
                left.value(wires).is_some()
            } else {
                true
            }
    }

    fn evaluate(&self, wires: &HashMap<String, u16>) -> u16 {
        let right = self.right.value(wires).unwrap();
        let left = if let Some(l) = &self.left {
            l.value(wires).unwrap_or(0)
        } else {
            0
        };
        match self.operation {
            Operation::Not => !right,
            Operation::Set => right,
            Operation::Lshift => left << right,
            Operation::Rshift => left >> right,
            Operation::And => left & right,
            Operation::Or => left | right,
        }
    }
}

fn run(setup: &Vec<(Connection, String)>, mut wires: HashMap<String, u16>) -> u16 {
    while let Some((connection, output)) = setup
        .iter()
        .find(|(c, s)| !wires.contains_key(s) && c.prerequisites_ok(&wires))
    {
        let result = connection.evaluate(&wires);
        wires.insert(output.clone(), result);
    }

    wires.get(&"a".to_string()).unwrap().clone()
}

pub fn solve(input: &str) -> Solution {
    let setup: Vec<(Connection, String)> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" -> ").unwrap();
            (a.parse().unwrap(), b.to_string())
        })
        .collect();

    let p1 = run(&setup, HashMap::new());
    let mut m = HashMap::new();
    m.insert("b".to_string(), p1.clone());
    let p2 = run(&setup, m);
    Solution::new(p1, p2)
}

impl FromStr for Connection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() == 1 {
            let right = parts[0].parse()?;
            Ok(Connection {
                operation: Operation::Set,
                left: None,
                right,
            })
        } else if parts.len() == 2 {
            let right = parts[1].parse()?;
            Ok(Connection {
                operation: Operation::Not,
                left: None,
                right,
            })
        } else {
            let operation = parts[1].parse()?;
            let left = parts[0].parse()?;
            let right = parts[2].parse()?;
            Ok(Connection {
                operation,
                left: Some(left),
                right,
            })
        }
    }
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
            _ => Err("Unknown operation"),
        }
    }
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
