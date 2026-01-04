// https://adventofcode.com/2017/day/9

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let mut level = 0;
    let mut in_garbage = false;
    let mut cancel_next = false;
    let mut p1 = 0;
    let mut p2 = 0;

    for c in input.chars() {
        if cancel_next {
            cancel_next = false;
        } else if in_garbage {
            match c {
                '!' => { cancel_next = true; },
                '>' => { in_garbage = false; },
                _ => { p2 += 1; },
            }
        } else {
            match c {
                '{' => { level += 1; p1 += level; },
                '}' => { level -= 1; },
                '<' => { in_garbage = true; },
                _ => {},
            }
        }

    }

    Solution::new(p1,p2)
}
