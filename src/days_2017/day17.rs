// https://adventofcode.com/2017/day/17

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let step = input.trim().parse::<usize>().unwrap();

    let mut buffer = vec![0];
    let mut position = 0;
    for i in 1..=2017 {
        let n = ((position + step) % buffer.len()) + 1;
        buffer.insert(n, i);
        position = n;
    }
    let p1 = buffer
        .iter()
        .cycle()
        .skip_while(|i| **i != 2017)
        .skip(1)
        .next()
        .unwrap();

    let p2 = (1..=50000000)
        .scan(0, |position, value| {
            *position = ((*position + step) % value) + 1;
            Some((*position, value))
        })
        .filter_map(|(position, value)| if position == 1 { Some(value) } else { None })
        .last()
        .unwrap();

    Solution::new(p1, p2)
}
