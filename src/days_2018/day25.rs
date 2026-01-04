// https://adventofcode.com/2018/day/11

use crate::common::Solution;
use itertools::Itertools;

pub fn solve(input: &str) -> Solution {
    let mut points: Vec<(i64, i64, i64, i64)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut p1 = 0;
    while let Some(start_point) = points.pop() {
        let mut investigate = vec![start_point];

        while let Some((x, y, z, t)) = investigate.pop() {
            while let Some(nearby_index) = points.iter().position(|(x2, y2, z2, t2)| {
                x.abs_diff(*x2) + y.abs_diff(*y2) + z.abs_diff(*z2) + t.abs_diff(*t2) <= 3
            }) {
                investigate.push(points.remove(nearby_index));
            }
        }
        p1 = p1 + 1;
    }

    let p2 = 0;
    Solution::new(p1, p2)
}
