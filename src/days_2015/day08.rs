// https://adventofcode.com/2015/day/8

use crate::common::Solution;

const BACKSLASH: u8 = '\\' as u8;
const DOUBLE_QUOTE: u8 = '"' as u8;
const HEX_MARKER: u8 = 'x' as u8;

pub fn solve(input: &str) -> Solution {
    let original_size = input.lines().map(|line| line.len()).sum::<usize>();

    let p1: usize = original_size - input
        .lines()
        .map(|line| {
            let b = line.as_bytes();
            let mut index = 1;
            let mut len = 0;
            while index < b.len() - 1 {
                index += match b[index] {
                    BACKSLASH if b[index + 1] == HEX_MARKER => 4,
                    BACKSLASH => 2,
                    _ => 1, };
                len += 1;
            }
            len
        })
        .sum::<usize>();

    let p2: usize = input
        .lines()
        .map(|line| 2 + line.as_bytes().iter()
                .map(|&b| match b {
                    BACKSLASH => 2,
                    DOUBLE_QUOTE => 2,
                    _ => 1,
                })
                .sum::<usize>())
        .sum::<usize>() - original_size;

    Solution::new(p1, p2)
}
