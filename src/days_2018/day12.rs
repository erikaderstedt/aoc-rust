// https://adventofcode.com/2018/day/12

use std::{collections::HashMap, convert::TryInto};

use itertools::Itertools;

use crate::common::Solution;
const SZ: usize = 1000;

pub fn solve(input: &str) -> Solution {
    let mut world = ['.'; SZ];

    let (first_line, rest) = input.split_once("\n\n").unwrap();
    let initial_state_text = first_line.trim().split(' ').skip(2).next().unwrap();
    for (a, b) in world
        .iter_mut()
        .skip(SZ / 2)
        .zip(initial_state_text.chars())
    {
        *a = b;
    }

    let z: HashMap<[char; 5], char> = rest
        .lines()
        .map(|line| {
            let (d1, d2) = line.split_once(" => ").unwrap();
            (
                d1.chars().collect_array().unwrap(),
                d2.chars().next().unwrap(),
            )
        })
        .collect();

    let mut p1 = 0;
    let mut vals = vec![];
    let p2 = loop {
        let mut new_world = world.clone();
        for i in 0..(SZ - 5) {
            let state: [char; 5] = world[i..(i + 5)].try_into().unwrap();
            let c = z[&state];
            new_world[i + 2] = c;
        }

        world = new_world;
        let q: i64 = world
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == '#')
            .map(|(i, _)| (i as i64) - ((SZ as i64) / 2))
            .sum();
        vals.push(q);
        if vals.len() == 20 {
            p1 = q;
        }
        let l = vals.len();
        // At iteration 113, the value increases by 80 each iteration.
        // The answer is (50000000000 - 113)*80+10520
        if l >= 100 && (vals[l - 1] - vals[l - 2] == vals[l - 2] - vals[l - 3]) {
            break (50_000_000_000 - (l as i64)) * (vals[l - 1] - vals[l - 2]) + vals[l - 1];
        }
    };

    Solution::new(p1, p2)
}
