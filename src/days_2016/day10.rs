// https://adventofcode.com/2016/day/10

use crate::common::Solution;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Destination {
    Bot(usize),
    Output(usize),
}

impl Destination {
    fn execute(
        &self,
        value: usize,
        bots: &mut Vec<(Option<usize>, Option<usize>)>,
        outputs: &mut [Option<usize>; 24],
    ) {
        match self {
            Destination::Bot(b) => {
                if bots[*b].0 == None {
                    bots[*b].0 = Some(value);
                } else if bots[*b].1 == None {
                    bots[*b].1 = Some(value);
                } else {
                    panic!("Giving to bot, but it has its hands full");
                }
            }
            Destination::Output(a) => {
                outputs[*a] = Some(value);
            }
        };
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rule {
    Assign(usize, usize),
    Give(usize, Destination, Destination),
}

pub fn solve(input: &str) -> Solution {
    let mut rules: Vec<Rule> = input
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line
                .split(' ')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            let inout: Vec<&str> = line.split(' ').skip(5).step_by(5).collect();
            if parts.len() == 2 {
                Rule::Assign(parts[0], parts[1])
            } else {
                let low = if inout[0] == "output" {
                    Destination::Output(parts[1])
                } else {
                    Destination::Bot(parts[1])
                };
                let high = if inout[1] == "output" {
                    Destination::Output(parts[2])
                } else {
                    Destination::Bot(parts[2])
                };
                Rule::Give(parts[0], low, high)
            }
        })
        .collect();

    let mut outputs = [None; 24];
    let mut bots: Vec<(Option<usize>, Option<usize>)> = vec![(None, None); 300];
    let mut p1 = None;
    while p1.is_none() || outputs[0].is_none() || outputs[1].is_none() || outputs[2].is_none() {
        let mut rule_index = 0;
        while rule_index < rules.len() {
            match rules[rule_index] {
                Rule::Assign(value, bot_index) => {
                    if bots[bot_index].0 == None {
                        bots[bot_index].0 = Some(value.clone());
                    } else if bots[bot_index].1 == None {
                        bots[bot_index].1 = Some(value.clone());
                    } else {
                        panic!("Assigning to bot, but it has its hands full");
                    }
                    rules.remove(rule_index);
                }
                Rule::Give(bot_index, d1, d2) => match bots[bot_index] {
                    (Some(a), Some(b)) => {
                        let low = a.min(b).clone();
                        let high = a.max(b).clone();
                        if low == 17 && high == 61 {
                            p1 = Some(bot_index.clone());
                        }
                        d1.execute(low, &mut bots, &mut outputs);
                        d2.execute(high, &mut bots, &mut outputs);
                        rules.remove(rule_index);
                    }
                    _ => {
                        rule_index += 1;
                    }
                },
            }
        }
    }

    let p1 = p1.unwrap();
    let p2: usize = outputs.iter().take(3).map(|o| o.unwrap()).product();

    Solution::new(p1, p2)
}
