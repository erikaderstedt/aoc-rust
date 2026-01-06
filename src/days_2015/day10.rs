// https://adventofcode.com/2015/day/10

use crate::common::Solution;
use itertools::Itertools;

fn look_and_say(inp: Vec<u8>) -> Vec<u8> {
    inp.into_iter()
        .dedup_with_count()
        .map(|(n, value)| {
            assert!(n < 10); // In the look and say sequence n is at most 3.
            [n as u8 + '0' as u8, value.clone()]
        })
        .flatten()
        .collect()
}

pub fn solve(input: &str) -> Solution {
    let mut a = input.trim().as_bytes().to_vec();
    for _ in 0..40 {
        a = look_and_say(a);
    }
    let p1 = a.len();

    // My input is Yb
    // https://mathworld.wolfram.com/CosmologicalTheorem.html
    // Answer is Eu.Ca.Ac.H.Ca.312

    for _ in 40..50 {
        a = look_and_say(a);
    }
    let p2 = a.len();

    Solution::new(p1, p2)
}
