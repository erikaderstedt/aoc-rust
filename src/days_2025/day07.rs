// https://adventofcode.com/2025/day/7

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let first_line = input.lines().next().unwrap();
    let start_position = first_line.find('S').unwrap();
    let n = first_line.len();

    let mut beams = vec![0; n];
    beams[start_position] = 1;

    let mut p1 = 0;
    for row in input.lines().skip(2).step_by(2) {
        let mut b2 = vec![0; n];
        for (i, n) in beams.iter().enumerate() {
            if *n == 0 { continue; }
            if row.as_bytes()[i] == b'^' {
                b2[i-1] += n;
                b2[i+1] += n;
                p1 = p1 + 1;
            } else {
                b2[i] += n;
            }
        }
        beams = b2;
    }
    let p2: usize = beams.iter().sum::<usize>();

    Solution::new(p1, p2)
}

