// https://adventofcode.com/2023/day/19

use std::collections::HashMap;

use crate::common::Solution;

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum Destination<'a> {
    Rejected,
    Accepted,
    Workflow(&'a str),
}

#[derive(Debug)]
enum Condition {
    Always,
    LessThan(u8,usize),
    GreaterThan(u8,usize),
}

#[derive(Debug)]
struct Rule<'a> {
    destination: Destination<'a>,
    condition: Condition,
}

#[derive(Debug, Clone, Copy)]
enum Limit {
    LessThan(usize),
    LessThanOrEqual(usize),
    GreaterThan(usize),
    GreaterThanOrEqual(usize)
}

#[derive(Debug, Clone)]
struct Search {
    h: HashMap<u8, Vec<Limit>>,
}

impl Search {

    fn start() -> Search {
        Search { h: vec![(b'x', vec![]), (b'm', vec![]), (b'a', vec![]), (b's', vec![])].into_iter().collect() }
    }

    fn inserting(&self, key: u8, limit: Limit) -> Self {
        let mut v = self.h.clone();
        match v.get_mut(&key) {
            Some(l) => { l.push(limit); },
            None => unreachable!(),
        }
        Search { h: v }
    }

    fn total(&self) -> usize {
        self.h.iter()
            .map(|(_,limits)| 
                    (1..=4000).filter(|value| limits.iter().all(|limit| match limit {
                        Limit::GreaterThan(check) => value > check,
                        Limit::GreaterThanOrEqual(check) => value >= check,
                        Limit::LessThan(check) => value < check,
                        Limit::LessThanOrEqual(check) => value <= check,
                    })).count())
            .product()
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
    
    let workflows: HashMap<&str, Vec<Rule>> = s1.lines()
    .map(|line| -> (&str, Vec<Rule>) {
        let (name, rules) = line.split_once('{').unwrap();        
        let (rules, _) = rules.split_once('}').unwrap();
        
        let rules: Vec<Rule> = rules.split(',').map(|s|
            match s.as_bytes()[0] {
                b'A' => Rule { condition: Condition::Always, destination: Destination::Accepted },
                b'R' => Rule { condition: Condition::Always, destination: Destination::Rejected },
                b'x' | b'm' | b'a' | b's' => {
                    let part = s.as_bytes()[0].clone(); 
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

        if rules.iter().all(|r| r.destination == Destination::Accepted) {
            (name, vec![Rule { condition: Condition::Always, destination: Destination::Accepted }])
        } else if rules.iter().all(|r| r.destination == Destination::Rejected) {
            (name, vec![Rule { condition: Condition::Always, destination: Destination::Rejected }])
        } else {
            (name, rules)
        }
    }).collect();

    
    let p1: usize = s2.lines().map(|line| -> usize {
        let h:HashMap<u8,usize> = line[1..line.len()-1].split(',')
            .map(|s| (s.as_bytes()[0].clone(), s[2..].parse::<usize>().unwrap())).collect();

        let mut workflow = "in";
        loop {
            let w = workflows.get(workflow).unwrap();
            let destination = w.iter().find_map(|rule| 
                match rule.condition {
                    Condition::Always => { Some(rule.destination)},
                    Condition::GreaterThan(p,value) if h[&p] > value => { Some(rule.destination) },
                    Condition::LessThan(p,value) if h[&p] < value => { Some(rule.destination) },
                    _ => None,
                }).unwrap();
            match destination {
                Destination::Accepted => break h.values().sum(),
                Destination::Rejected => break 0,
                Destination::Workflow(x) => workflow = x,
            };
        }
    }).sum();

    let s = Search::start();

    let p2 = s.apply(&workflows["in"], 0, &workflows);

    Solution::new(p1, p2)
}

