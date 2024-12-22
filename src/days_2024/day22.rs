// https://adventofcode.com/2024/day/22

use std::collections::{HashMap, HashSet};

use crate::common::Solution;

fn evolve(i: &usize) -> usize {
    let a = ((i << 6) ^ i) & ((1 << 24) - 1);
    let b = ((a >> 5) ^ a) & ((1 << 24) - 1);
    let c = ((b << 11) ^ b) & ((1 << 24) - 1);
    c
}

type Sequence = [i8;4];

pub fn solve(input: &str) -> Solution {
    let seeds: Vec<usize> = input.lines().map(|line| line.trim().parse::<usize>().unwrap()).collect();

    let mut totals: HashMap<Sequence, isize> = HashMap::new(); 
    let mut p1 = 0;

    for seed in seeds.iter() {
        let mut x = seed.clone();
        let mut h: HashSet<Sequence> = HashSet::new();
        let mut prices = [0i8;2000];

        for i in 0..2000 {
            let current_price = (x % 10) as i8;
            prices[i] = current_price;
            if prices[i] != 0 && i >= 4 {
                let d1 = prices[i] - prices[i-1];
                let d2 = prices[i-1] - prices[i-2];
                let d3 = prices[i-2] - prices[i-3];
                let d4 = prices[i-3] - prices[i-4];
                // a sequence of d4,d3,d2,d1 is worth what it is *the first time it is encountered*
                let k = [d4,d3,d2,d1];
                if !h.contains(&k) {
                    h.insert(k);
                    *totals.entry(k).or_default() += current_price as isize;
                }
            }
            x = evolve(&x);
        }
        p1 += x;
    }

    let p2 = totals.values().max().unwrap();

    Solution::new(p1, p2)
}
