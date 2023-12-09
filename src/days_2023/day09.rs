// https://adventofcode.com/2023/day/9

use crate::common::Solution;

fn extrapolated(mut values: Vec<i64>) -> (i64, i64) {
    let first = values[0];
    if values.iter().all(|x| *x == first) {
        (first, first)
    } else {
        for i in 0..values.len()-1 { values[i] = values[i+1]-values[i] };
        let last = values.pop().unwrap();
        let (first_below, last_below) = extrapolated(values);
        (first - first_below, last + last_below)
    }
}

pub fn solve(input: &str) -> Solution {
    let (p1,p2) = input
        .lines()
        .map(|line| extrapolated(line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect()))
        .fold((0,0), |(p1,p2), (first, last)| (p1 + last, p2 + first));

    Solution::new(p1,p2)
}
