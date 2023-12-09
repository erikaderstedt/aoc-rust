// https://adventofcode.com/2023/day/9

use crate::common::Solution;

fn extrapolated_sequence(v: Vec<i64>) -> (i64, i64) {
    let m: Vec<i64> = v.windows(2).map(|i| i[1] - i[0]).collect();
    if m.iter().all(|x| *x == 0) {
        (v[0].clone(), v[0].clone())
    } else {
        let q = extrapolated_sequence(m);
        (v.first().unwrap() - q.0, q.1 + v.last().unwrap())
    }
}

pub fn solve(input: &str) -> Solution {
    let new_values: Vec<(i64,i64)> = input.lines().map(|line| {
        let v: Vec<i64> = line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect();
        extrapolated_sequence(v)
    }).collect();

    let p1: i64 = new_values.iter().map(|v| v.1).sum();
    let p2: i64 = new_values.iter().map(|v| v.0).sum();

    Solution::new(p1,p2)
}
