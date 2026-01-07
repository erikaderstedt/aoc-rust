// https://adventofcode.com/2015/day/1

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {

    let (m1, m2) = input
        .chars()
        .enumerate()    
        .fold((0i32, Option::None), |(floor, reached), (index, p)| {
            let new_floor = floor + if p == '(' { 1 } else { -1 };
            (new_floor, if reached == Option::None && new_floor == -1 { Some(index) } else { reached })
        });

    Solution::new(m1, m2.unwrap() + 1)
}