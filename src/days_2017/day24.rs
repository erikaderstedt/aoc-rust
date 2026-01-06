// https://adventofcode.com/2017/day/24

use crate::common::Solution;
use itertools::Itertools;

type Connector = (usize, usize);

fn look_for_strongest(position: usize, connectors: Vec<Connector>) -> usize {
    (0..connectors.len())
        .filter(|&i| connectors[i].0 == position || connectors[i].1 == position)
        .map(|i| {
            let mut b = connectors.clone();
            let c = b.remove(i);
            let new_position = if c.0 == position { c.1 } else { c.0 };
            let sum = c.0 + c.1;
            sum + look_for_strongest(new_position, b)
        })
        .max()
        .unwrap_or(0)
}

fn look_for_longest(position: usize, connectors: Vec<Connector>) -> (usize, usize) {
    (0..connectors.len())
        .filter(|&i| connectors[i].0 == position || connectors[i].1 == position)
        .map(|i| {
            let mut b = connectors.clone();
            let c = b.remove(i);
            let new_position = if c.0 == position { c.1 } else { c.0 };
            let sum = c.0 + c.1;
            let w = look_for_longest(new_position, b);
            (sum + w.0, 1 + w.1)
        })
        .max_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        })
        .unwrap_or((0, 0))
}

pub fn solve(input: &str) -> Solution {
    let connectors: Vec<Connector> = input
        .lines()
        .map(|line| {
            line.split('/')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let p1 = look_for_strongest(0, connectors.clone());
    let p2 = look_for_longest(0, connectors).0;

    Solution::new(p1, p2)
}
