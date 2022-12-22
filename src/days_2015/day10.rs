// https://adventofcode.com/2015/day/10

use crate::common::Solution;
use itertools::Itertools;

fn look_and_say(inp: Vec<u8>) -> Vec<u8> {
    inp.iter()
        .dedup_with_count()
        .map(|(n, value)| vec![n as u8,*value])
        .flatten()
        .collect()
}

pub fn solve(input: &str) -> Solution {
    
    // DOES NOT WORK

    let mut a = input.as_bytes().to_vec();
    for _ in 0..40 {
        a = look_and_say(a);
    }
    let p1 = a.len();

    for _ in 40..50 {
        a = look_and_say(a);
    }
    let p2 = a.len();

    Solution::new(p1,p2)
}
