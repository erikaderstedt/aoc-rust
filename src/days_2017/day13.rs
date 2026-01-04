// https://adventofcode.com/2017/day/13

use crate::common::Solution;
use itertools::Itertools;

pub fn solve(input: &str) -> Solution {
    let mut scanners: Vec<(usize, usize, usize)> = vec![];

    for line in input.lines() {
        let (depth, range) = line
            .split(": ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let period = 2 * (range - 1);
        scanners.push((depth, range, period));
    }

    let p1 = scanners
        .iter()
        .filter(|(depth, _, period)| depth % period == 0)
        .map(|(depth, range, _)| depth * range)
        .sum::<usize>();

    let p2 = (0..)
        .find(|delay| {
            scanners
                .iter()
                .all(|(depth, _, period)| (delay + depth) % period != 0)
        })
        .unwrap();

    Solution::new(p1, p2)
}
