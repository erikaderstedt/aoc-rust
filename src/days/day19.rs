use crate::common::Solution;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq,Eq)]
enum Rule {
    Character(char),
    Single(Vec<usize>),
    Multiple((Vec<usize>, Vec<usize>)),
}

type RulesList = HashMap<usize,Rule>;

impl Rule {
    // Returns the possible different remaining strings
    fn matches<'a>(&self, rules: &RulesList, s: &'a str) -> Vec<&'a str> {
        match self {
            Rule::Character(c) => match s.chars().next() {
                Some(c1) if c1 == *c => vec![&s[1..]],
                _ => vec![],
            },
            Rule::Single(inner) => {
                let mut r = vec![s];
                let mut r2;
                for rule in inner {
                    r2 = Vec::new();
                    for r in &r {
                        let mut next_results = rules[rule].matches(rules, r);
                        r2.append(&mut next_results);
                    }
                    if r2.len() == 0 { return vec![]; }
                    r = r2;
                }
                r
            },
            Rule::Multiple((left, right)) => {
                let mut r1 = Rule::Single(left.clone()).matches(rules, s);
                let r2 = Rule::Single(right.clone()).matches(rules, s);
                r1.extend(r2);
                r1
            }
        }
    }
}

impl std::str::FromStr for Rule {
    type Err = &'static str;

    fn from_str(ch: &str) -> std::result::Result<Rule,Self::Err> {
        Ok(if ch.contains("\"") {
            Rule::Character(ch.chars().skip(1).next().unwrap())
        } else if ch.contains("|") {
            let (a,b) = ch.split('|').collect_tuple().unwrap();
            Rule::Multiple((a.split(' ').filter(|c| c.len() > 0).map(|c| c.parse::<usize>().unwrap()).collect(),
            b.split(' ').filter(|c| c.len() > 0).map(|c| c.parse::<usize>().unwrap()).collect()))
        } else {
            Rule::Single(ch.split(' ').filter(|c| c.len() > 0).map(|c| c.parse::<usize>().unwrap()).collect())
        })
    }
}

fn number_of_matches_rule_0(input: &str, rules: &RulesList) -> usize {
    input.lines().skip_while(|&line| line != "").skip(1).filter(|line| {
        rules[&0].matches(&rules,line).iter().any(|v| v.len() == 0)
    })
    .count()
}

pub fn solve(input: &str) -> Solution {
    let mut rules: RulesList = input.lines().take_while(|&line| line != "").map(|line| {
        let (num, ch) = line.split(": ").collect_tuple().unwrap();
        let num = num.parse::<usize>().unwrap();
        let rule = ch.parse::<Rule>().unwrap();
        (num, rule)
    }).collect();

    let p1 = number_of_matches_rule_0(input, &rules);

    rules.insert(8, "42 | 42 8".parse::<Rule>().unwrap());
    rules.insert(11, "42 31 | 42 11 31".parse::<Rule>().unwrap());

    let p2 = number_of_matches_rule_0(input, &rules);

    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}
