// https://adventofcode.com/2015/day/3

use crate::common::Solution;
use md5;

fn find_adventcoin(input: &str, prefix: &str) -> usize {
    let mut counter = 0;

    loop {
        counter += 1;
        let test_key = format!("{}{}", input, counter);
        let digest = md5::compute(test_key);
        let s = format!("{:x}", digest);
        if s.starts_with(prefix) { break }
    }
    counter
}

pub fn solve(input: &str) -> Solution {
    let m1 = find_adventcoin(input, "00000");
    let m2 = find_adventcoin(input, "000000");

    Solution::new(m1, m2)
}