// https://adventofcode.com/2017/day/7

use itertools::Itertools;

use crate::common::Solution;

#[derive(Debug)]
struct Program<'a> {
    base: &'a str,
    weight: i64,
    top: Vec<&'a str>,
}

// Either return weight difference, or total weight

#[derive(Debug)]
enum CheckResult {
    Difference(i64),
    TotalWeight(i64),
}

impl<'a> Program<'a> {
    fn check(&self, programs: &Vec<Program>) -> CheckResult {
        if self.top.len() == 0 {
            CheckResult::TotalWeight(self.weight)
        } else {
            // Either all on top are the same,
            // or one is a different TotalWeight
            // or one is Difference

            let mut weights = vec![];
            for t in self.top.iter() {
                let p = programs.iter().find(|p| p.base == *t).unwrap();
                match p.check(programs) {
                    CheckResult::Difference(x) => return CheckResult::Difference(x),
                    CheckResult::TotalWeight(tw) => {
                        weights.push(tw);
                    }
                }
            }
            if weights.iter().all(|w| *w == weights[0]) {
                CheckResult::TotalWeight(self.weight + weights.iter().sum::<i64>())
            } else {
                assert!(weights.len() > 2);
                let correct_weight = weights
                    .iter()
                    .counts()
                    .drain()
                    .filter(|c| c.1 != 1)
                    .map(|c| c.0)
                    .cloned()
                    .next()
                    .unwrap();

                let should_have_been = weights
                    .iter()
                    .zip(self.top.iter())
                    .filter(|(w, _)| **w != correct_weight)
                    .map(|(w, t)| {
                        let p = programs.iter().find(|p| p.base == *t).unwrap();
                        p.weight + correct_weight - w
                    })
                    .next()
                    .unwrap();

                CheckResult::Difference(should_have_been)
            }
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let programs: Vec<Program> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let base = parts[0];
            let weight = parts[1][1..(parts[1].len() - 1)].parse::<i64>().unwrap();
            if let Some(s) = line.split(" -> ").skip(1).next() {
                Program {
                    base,
                    weight,
                    top: s.split(", ").collect(),
                }
            } else {
                Program {
                    base,
                    weight,
                    top: vec![],
                }
            }
        })
        .collect();

    let p1 = programs
        .iter()
        .find_map(|program| {
            if programs.iter().any(|p| p.top.contains(&program.base)) {
                None
            } else {
                Some(program.base)
            }
        })
        .unwrap()
        .to_string();

    let base_program = programs.iter().find(|p| p.base == p1.as_str()).unwrap();

    let p2 = match base_program.check(&programs) {
        CheckResult::Difference(x) => x,
        CheckResult::TotalWeight(_) => panic!("Could not find imbalance"),
    };

    Solution::new(p1, p2)
}
