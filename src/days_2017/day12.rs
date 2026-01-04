// https://adventofcode.com/2017/day/12

use crate::common::Solution;
use itertools::Itertools;

pub fn solve(input: &str) -> Solution {
    let mut data: Vec<(usize, Vec<usize>)> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (left_program, programs) = line.split(" <-> ").collect_tuple().unwrap();
            let left = left_program.parse::<usize>().unwrap();
            assert!(left == i);
            (
                left,
                programs
                    .split(", ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect(),
            )
        })
        .collect();

    let mut p1 = 0;
    let mut p2 = 0;
    while let Some(start_point) = data.pop() {
        let mut group_contains_zero = false;
        let mut investigate = vec![start_point];
        let mut n = 0;
        while let Some(p) = investigate.pop() {
            n = n + 1;
            if p.0 == 0 {
                group_contains_zero = true;
            }

            for program in p.1 {
                if let Some(i) = data.iter().position(|q| q.0 == program) {
                    investigate.push(data.remove(i));
                }
            }
        }

        if group_contains_zero {
            p1 = n;
        }
        p2 += 1;
    }
    Solution::new(p1, p2)
}
