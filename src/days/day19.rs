use crate::common::Solution;
use itertools::Itertools;
use std::collections::HashMap;

enum Rule {
    Character(u8),
    Single(Vec<usize>),
    Multiple((Vec<usize>, Vec<usize>)),
}

impl std::str::FromStr for Rule {
    type Err = &'static str;

    fn from_str(ch: &str) -> std::result::Result<Rule,Self::Err> {
        Ok(if ch.contains("\"") {
            Rule::Character(ch.as_bytes()[1])
        } else if ch.contains("|") {
            let (a,b) = ch.split('|').collect_tuple().unwrap();
            Rule::Multiple((a.split(' ').filter(|c| c.len() > 0).map(|c| c.parse::<usize>().unwrap()).collect(),
            b.split(' ').filter(|c| c.len() > 0).map(|c| c.parse::<usize>().unwrap()).collect()))
        } else {
            Rule::Single(ch.split(' ').filter(|c| c.len() > 0).map(|c| c.parse::<usize>().unwrap()).collect())
        })
    }
}

struct RulesList {
    rules: HashMap<usize,Rule>
}

impl RulesList {

    fn match_sequence<'a>(&self, indices: &Vec<usize>, s: &'a [u8], q: &mut Vec<usize>) -> bool {
        q.extend(indices.iter().rev());
        self.match_on(s, q)
    }

    fn match_on<'a>(&self, s: &'a [u8], mut q: &mut Vec<usize>) -> bool {
        match q.pop() {
            Some(rule_index) => match &self.rules[&rule_index] {
                Rule::Character(c1) => matches!(s.get(0), Some(c) if c == c1) && self.match_on(&s[1..], q),
                Rule::Single(indices) => self.match_sequence(&indices, s, &mut q),
                Rule::Multiple((left,right)) => {
                    self.match_sequence(&left, s, &mut q.clone()) ||
                    self.match_sequence(&right, s, &mut q.clone())
                }
            }
            None => s.len() == 0,
        }
    }

    fn matches_to_rule_0(&self, messages: &Vec<&str>) -> usize {
        messages.iter()
            .filter(|line| {
                let mut q = vec![0];
                self.match_on(line.as_bytes(), &mut q)
            })
            .count()
    }
}

pub fn solve(input: &str) -> Solution {
    let mut matcher = RulesList {
        rules:  input.lines().take_while(|&line| line != "").map(|line| {
                    let (num, ch) = line.split(": ").collect_tuple().unwrap();
                    let num = num.parse::<usize>().unwrap();
                    let rule = ch.parse::<Rule>().unwrap();
                    (num, rule)
                }).collect(),
    };

    let messages: Vec<&str> = input.lines()
        .skip_while(|&line| line != "")
        .skip(1).collect();

    let p1 = matcher.matches_to_rule_0(&messages);

    matcher.rules.insert(8, Rule::Multiple((vec![42], vec![42,8])));
    matcher.rules.insert(11, Rule::Multiple((vec![42,31], vec![42,11,31])));
    
    let p2 = matcher.matches_to_rule_0(&messages);

    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}
