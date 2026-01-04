// https://adventofcode.com/2017/day/1

use crate::common::Solution;

const ZERO: u8 = '0' as u8;

fn calculate_inverse_captcha(s: &str, offset: usize) -> usize {
    s
        .chars()
        .zip(s.chars().cycle().skip(offset))
        .filter(|(a,b)| a == b)
        .map(|(a,_)| ((a as u8) - ZERO) as usize)
        .sum::<usize>()
}
pub fn solve(input: &str) -> Solution {
    let s = input.trim();
    let p1 = calculate_inverse_captcha(s, 1);
    let p2 = calculate_inverse_captcha(s, s.len()/2);
    Solution::new(p1,p2)
}
