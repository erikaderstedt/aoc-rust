// https://adventofcode.com/2024/day/5

use crate::common::Solution;
use itertools::Itertools;

type Update = Vec<usize>;

pub fn solve(input: &str) -> Solution {
    let (rule_input, update_input) = input.split_once("\n\n").unwrap();
    let rules: Vec<(usize, usize)> = rule_input
        .lines()
        .map(|line| {
            line.split('|')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let updates: Vec<Update> = update_input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let (correctly_ordered, incorrectly_ordered): (Vec<Update>, Vec<Update>) =
        updates.into_iter().partition(|update| {
            rules.iter().all(|(r1, r2)| {
                match (
                    update.iter().position(|x| x == r1),
                    update.iter().position(|x| x == r2),
                ) {
                    (Some(a), Some(b)) => a < b,
                    _ => true,
                }
            })
        });

    let p1 = correctly_ordered
        .into_iter()
        .map(|update| update[update.len() >> 1])
        .sum::<usize>();
    let p2 = incorrectly_ordered
        .into_iter()
        .map(|mut update| -> usize {
            while !rules.iter().all(|(r1, r2)| {
                match (
                    update.iter().position(|x| x == r1),
                    update.iter().position(|x| x == r2),
                ) {
                    (Some(a), Some(b)) if a > b => {
                        update.swap(a, b);
                        false
                    }
                    _ => true,
                }
            }) {}

            update[update.len() >> 1]
        })
        .sum::<usize>();

    Solution::new(p1, p2)
}
