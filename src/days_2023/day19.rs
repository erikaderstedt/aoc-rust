// https://adventofcode.com/2023/day/19

use std::collections::HashMap;
use crate::common::Solution;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Destination<'a> {
    Rejected,
    Accepted,
    Workflow(&'a str),
}

enum Condition {
    Always,
    LessThan(usize,usize),
    GreaterThan(usize,usize),
}

struct Rule<'a> {
    destination: Destination<'a>,
    condition: Condition,
}

#[derive(Clone, Copy)]
enum Limit {
    LessThan(usize),
    LessThanOrEqual(usize),
    GreaterThan(usize),
    GreaterThanOrEqual(usize)
}

struct Search {
    v: [std::ops::RangeInclusive<usize>; 4],
}

impl Search {
    fn new() -> Search {
        Search { v: [1..=4000, 1..=4000, 1..=4000, 1..=4000] }
    }

    fn impose_limit(range: &std::ops::RangeInclusive<usize>, limit: Limit) -> std::ops::RangeInclusive<usize> {
        match limit {
            Limit::LessThan(value) =>           *range.start()..=(value-1),
            Limit::LessThanOrEqual(value) =>    *range.start()..=value,
            Limit::GreaterThan(value) =>        (value+1)..=*range.end(),
            Limit::GreaterThanOrEqual(value) => value..=*range.end(),
        }
    } 

    fn inserting(&self, u: usize, limit: Limit) -> Self {
        let mut v = self.v.clone();
        v[u] = Search::impose_limit(&v[u], limit);
        Search { v }
    }

    fn total(&self) -> usize {
        // In our input there is no case where the conditions go to zero, so these if's are not really needed.
        self.v.iter().map(|r| if r.end() >= r.start() { r.end() - r.start() + 1 } else { 0 }).product()
    }

    fn apply(&self, current_workflow: &Vec<Rule>, index: usize, workflows: &HashMap<&str, Vec<Rule>>) -> usize {
        match current_workflow[index].condition {
            Condition::GreaterThan(u,value) => {
                let branch1 = self.inserting(u, Limit::LessThanOrEqual(value));
                let branch2 = self.inserting(u, Limit::GreaterThan(value));
                branch1.apply(current_workflow, index + 1, workflows) +
                match current_workflow[index].destination {
                    Destination::Accepted => branch2.total(),
                    Destination::Rejected => 0,
                    Destination::Workflow(w) => branch2.apply(&workflows[w], 0, workflows),
                }
            },
            Condition::LessThan(u, value ) => {
                let branch1 = self.inserting(u, Limit::GreaterThanOrEqual(value));
                let branch2 = self.inserting(u, Limit::LessThan(value));

                branch1.apply(current_workflow, index + 1, workflows) +
                match current_workflow[index].destination {
                    Destination::Accepted => branch2.total(),
                    Destination::Rejected => 0,
                    Destination::Workflow(w) => branch2.apply(&workflows[w], 0, workflows),
                }  
            },
            Condition::Always => {
                match current_workflow[index].destination {
                    Destination::Accepted => self.total(),
                    Destination::Rejected => 0,
                    Destination::Workflow(w) => self.apply(&workflows[w], 0, workflows),
                }
            }
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let (s1, s2) = input.split_once("\n\n").unwrap();
    
    let workflows: HashMap<&str, Vec<Rule>> = s1.lines().map(|line| -> (&str, Vec<Rule>) {
        let (name, rules) = line.split_once('{').unwrap();
        
        let rules: Vec<Rule> = rules[..rules.len()-1].split(',').map(|s|
            match s.as_bytes()[0] {
                b'A' => Rule { condition: Condition::Always, destination: Destination::Accepted },
                b'R' => Rule { condition: Condition::Always, destination: Destination::Rejected },
                b'x' | b'm' | b'a' | b's' => {
                    let part = match s.as_bytes()[0] {
                        b'x' => 0, b'm' => 1, b'a' => 2, b's' => 3, _ => unreachable!(),
                    };
                    let condition = match s.as_bytes()[1] {
                        b'<' => { 
                            let q: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
                            let n = q.parse::<usize>().unwrap();
                            Condition::LessThan(part, n)
                        },
                        b'>' => {
                            let q: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
                            let n = q.parse::<usize>().unwrap();
                            Condition::GreaterThan(part, n)
                        },
                        _ => Condition::Always,
                    };
                    let destination = if let Some((_,d)) = s.split_once(':') {
                        match d {
                            "A" => Destination::Accepted,
                            "R" => Destination::Rejected,
                            x => Destination::Workflow(x),
                        }
                    } else {
                        Destination::Workflow(s)
                    };
                    Rule { condition, destination }
                },
                _ => Rule { condition: Condition::Always, destination: Destination::Workflow(s) },
            }).collect();
        (name, rules)
    }).collect();
    
    let p1: usize = s2.lines().map(|line| -> usize {
        let h:Vec<usize> = line[1..line.len()-1].split(',')
            .map(|s| s[2..].parse::<usize>().unwrap()).collect();

        let mut workflow = "in";
        loop {
            let w = workflows.get(workflow).unwrap();
            let destination = w.iter().find_map(|rule| 
                match rule.condition {
                    Condition::Always => { Some(rule.destination)},
                    Condition::GreaterThan(p,value) if h[p] > value => { Some(rule.destination) },
                    Condition::LessThan(p,value) if h[p] < value => { Some(rule.destination) },
                    _ => None,
                }).unwrap();
            match destination {
                Destination::Accepted => break h.iter().sum(),
                Destination::Rejected => break 0,
                Destination::Workflow(x) => workflow = x,
            };
        }
    }).sum();

    let p2 = Search::new().apply(&workflows["in"], 0, &workflows);

    Solution::new(p1, p2)
}

