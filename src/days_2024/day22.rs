// https://adventofcode.com/2024/day/22

use std::collections::{HashMap, HashSet};

use crate::common::Solution;

fn evolve(i: &usize) -> usize {
    let a = ((i << 6) ^ i) & ((1 << 24) - 1);
    let b = ((a >> 5) ^ a) & ((1 << 24) - 1);
    let c = ((b << 11) ^ b) & ((1 << 24) - 1);
    c
}

pub fn solve(input: &str) -> Solution {
    let seeds: Vec<usize> = input.lines().map(|line| line.trim().parse::<usize>().unwrap()).collect();

    let p1 = seeds.iter().map(|seed| {
        let mut x = seed.clone();
        for _ in 0..2000 {
            x = evolve(&x);
        }
        x
    }).sum::<usize>();

    let data: Vec<HashMap<(i8,i8,i8,i8), i8>> = seeds.iter().map(|seed| {
        let mut x = seed.clone();
        let mut h: HashMap<(i8,i8,i8,i8), i8> = HashMap::new();
        let mut prices = [0i8;2000]; 
        for i in 0..2000 {
            prices[i] = (x % 10) as i8;
            if i >= 4 {
                let d1 = prices[i] - prices[i-1];
                let d2 = prices[i-1] - prices[i-2];
                let d3 = prices[i-2] - prices[i-3];
                let d4 = prices[i-3] - prices[i-4];
                // a sequence of d4,d3,d2,d1 is worth what it is *the first time it is encountered*
                let k = (d4,d3,d2,d1);
                if !h.contains_key(&k) {
                    h.insert(k, prices[i]);
                }
            }
            x = evolve(&x);
        }
        h
    }).collect();

    let all_diffs: HashSet<(i8,i8,i8,i8)> = data
        .iter()
        .map(|m| {
            m.keys()
        })
        .flatten()
        .cloned()
        .collect();

    println!("all diffs: {}", all_diffs.len());

    let p2 = all_diffs
        .iter()
        .map(|diff| {
            data
             .iter()
             .map(|m| 
                *m
                .get(diff)
                .unwrap_or(&0) as isize
            ).sum::<isize>()
        })
        .max().unwrap();

    Solution::new(p1, p2)
}
