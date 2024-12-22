// https://adventofcode.com/2024/day/22

use crate::common::Solution;

fn evolve(i: &usize) -> usize {
    let a = ((i << 6) ^ i) & ((1 << 24) - 1);
    let b = ((a >> 5) ^ a) & ((1 << 24) - 1);
    let c = ((b << 11) ^ b) & ((1 << 24) - 1);
    c
}

pub fn solve(input: &str) -> Solution {
    let seeds: Vec<usize> = input.lines().map(|line| line.trim().parse::<usize>().unwrap()).collect();

    let mut totals = [0isize; 19*19*19*19];
    let mut p1 = 0;

    for seed in seeds.iter() {
        let mut x = seed.clone();
        let mut seen = [false; 19*19*19*19];
        let mut prices = [0i8;2000];

        for i in 0..2000 {
            let current_price = (x % 10) as i8;
            prices[i] = current_price;
            if i >= 4 {
                let k = 
                ((prices[i  ] - prices[i-1] + 9) as usize)*19*19*19 +
                ((prices[i-1] - prices[i-2] + 9) as usize)*19*19 +
                ((prices[i-2] - prices[i-3] + 9) as usize)*19 +
                ((prices[i-3] - prices[i-4] + 9) as usize);

                // a sequence of d4,d3,d2,d1 is worth what it is *the first time it is encountered*
                if !seen[k] {
                    totals[k] += current_price as isize;
                    seen[k] = true;
                }
            }
            x = evolve(&x);
        }
        p1 += x;
    }

    let p2 = totals.iter().max().unwrap();

    Solution::new(p1, p2)
}
