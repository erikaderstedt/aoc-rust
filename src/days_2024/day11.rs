// https://adventofcode.com/2024/day/11

use std::collections::HashMap;

use crate::common::Solution;

fn blink(stones: HashMap<u64, usize>) -> HashMap<u64, usize> {
    stones
        .into_iter()
        .fold(HashMap::new(), |mut new_stones, (x, cnt)| {
            if x == 0 {
                *new_stones.entry(1).or_insert(0) += cnt;
            } else {
                let num_digits = x.ilog10() + 1;
                if num_digits % 2 == 0 {
                    *new_stones.entry(x % 10u64.pow(num_digits >> 1)).or_insert(0) += cnt;
                    *new_stones.entry(x / 10u64.pow(num_digits >> 1)).or_insert(0) += cnt;
                } else {
                    *new_stones.entry(x * 2024).or_insert(0) += cnt;
                }
            }
            new_stones
        })
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
