// https://adventofcode.com/2015/day/13

use crate::common::Solution;
use itertools::Itertools;

fn find_optimal(deltas: &Vec<Vec<i64>>) -> i64 {
    let n = deltas.len();
    (0..n)
        .permutations(n)
        .map(|i| {
            i.into_iter()
                .circular_tuple_windows()
                .map(|(a, b)| deltas[a][b] + deltas[b][a])
                .sum::<i64>()
        })
        .max()
        .unwrap()
}

pub fn solve(input: &str) -> Solution {
    let relations: Vec<(&str, i64, &str)> = input
        .lines()
        .map(|line| {
            let (subject, _, gain_or_lose, units, _, _, _, _, _, _, object) = line
                [0..(line.len() - 1)]
                .split(' ')
                .collect_tuple()
                .unwrap();
            let delta = units.parse::<i64>().unwrap() * if gain_or_lose == "gain" { 1 } else { -1 };
            (subject, delta, object)
        })
        .collect();

    let names: Vec<&str> = relations.iter().map(|r| r.0).dedup().collect();
    let n = names.len();
    let mut deltas: Vec<Vec<i64>> = vec![vec![0; n]; n];
    for r in relations.iter() {
        let i = names.iter().position(|p| *p == r.0).unwrap();
        let j = names.iter().position(|p| *p == r.2).unwrap();
        deltas[i][j] = r.1;
    }

    let p1 = find_optimal(&deltas);

    for i in deltas.iter_mut() {
        i.push(0);
    }
    deltas.push(vec![0; n + 1]);

    let p2 = find_optimal(&deltas);

    Solution::new(p1, p2)
}
