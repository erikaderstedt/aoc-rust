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

type RulesList = HashMap<usize,Rule>;

impl Rule {

    fn process_indices<'a>(rule_indices: &Vec<usize>, rules: &RulesList, s: &'a [u8]) -> Vec<&'a [u8]> {
        rule_indices.iter().fold(vec![s], |acc, rule_index| {
            acc.into_iter().map(|r| {
                rules[rule_index].matches(rules, r)
            }).flatten().collect()
        })
    }
    // Returns the possible different remaining strings after matching this rule.
    fn matches<'a>(&self, rules: &RulesList, s: &'a [u8]) -> Vec<&'a [u8]> {
        match self {
            Rule::Character(c) => match s.get(0) {
                Some(c1) if c == c1 => { vec![&s[1..]] }
                _ => vec![],
            },
            Rule::Single(rule_indices) => Rule::process_indices(rule_indices, rules, s),
            Rule::Multiple((left, right)) => {
                let mut r1 = Rule::process_indices(left, rules, s);
                r1.extend(Rule::process_indices(right, rules, s));
                r1
            },
        }
    }
}

fn number_of_matches_rule_0(messages: &Vec<&str>, rules: &RulesList) -> usize {
    messages.iter()
        .filter(|line| {
            rules[&0].matches(&rules,&line.as_bytes()).iter().any(|v| v.len() == 0)
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

    let messages: Vec<&str> = input.lines()
        .skip_while(|&line| line != "")
        .skip(1).collect();

    let p1 = number_of_matches_rule_0(&messages, &rules);

    rules.insert(8, "42 | 42 8".parse::<Rule>().unwrap());
    rules.insert(11, "42 31 | 42 11 31".parse::<Rule>().unwrap());

    let p2 = number_of_matches_rule_0(&messages, &rules);

    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}
