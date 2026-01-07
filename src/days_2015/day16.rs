// https://adventofcode.com/2015/day/16

use crate::common::Solution;
use itertools::Itertools;
use std::collections::HashMap;

const TICKER_TAPE: &str = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";

fn entry(s: &str) -> (&str, usize) {
    let (key, value) = s
        .split(": ")
        .skip_while(|s| s.starts_with("Sue"))
        .collect_tuple()
        .unwrap();
    let value = value.parse::<usize>().unwrap();
    (key, value)
}

pub fn solve(input: &str) -> Solution {
    let aunt_sues: Vec<HashMap<&str, usize>> = input
        .lines()
        .map(|line| line.split(", ").map(|c| entry(c)).collect())
        .collect();

    let ticker_tape: HashMap<&str, usize> = TICKER_TAPE.lines().map(|c| entry(c)).collect();

    let p1 = aunt_sues
        .iter()
        .position(|sue| {
            ticker_tape
                .iter()
                .all(|(key, value)| sue.get(*key).unwrap_or(value) == value)
        })
        .unwrap()
        + 1;
    let p2 = aunt_sues
        .iter()
        .position(|sue| {
            ticker_tape.iter().all(|(key, value)| {
                if let Some(sues_value) = sue.get(*key) {
                    match *key {
                        "cats" | "trees" => sues_value > value,
                        "pomeranians" | "goldfish" => sues_value < value,
                        _ => sues_value == value,
                    }
                } else {
                    true
                }
            })
        })
        .unwrap()
        + 1;

    Solution::new(p1, p2)
}
