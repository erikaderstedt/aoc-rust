// https://adventofcode.com/2024/day/11

use std::collections::HashMap;

use crate::common::Solution;

fn blink(stones: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut n1: HashMap<u64, usize> = HashMap::new();

    for (x, cnt) in stones.into_iter() {
        if x == 0 {
            *n1.entry(1).or_default() += cnt;
            continue;
        }

        let s = x.to_string();
        let l = x.to_string().len();

        if l % 2 == 0 {
            let (a1, a2) = (
                s[..(l >> 1)].parse::<u64>().unwrap(),
                s[(l >> 1)..].parse::<u64>().unwrap(),
            );
            *n1.entry(a1).or_default() += cnt;
            *n1.entry(a2).or_default() += cnt;
        } else {
            let a1 = x * 2024;
            *n1.entry(a1).or_default() += cnt;
        }
    }
    n1
}

pub fn solve(input: &str) -> Solution {
    let mut n: HashMap<u64, usize> = input
        .split(' ')
        .map(|s| (s.trim().parse::<u64>().unwrap(), 1))
        .collect();

    for _ in 0..25 {
        n = blink(n);
    }

    let p1 = n.values().sum::<usize>();

    for _ in 25..75 {
        n = blink(n);
    }
    let p2 = n.values().sum::<usize>();

    Solution::new(p1, p2)
}
