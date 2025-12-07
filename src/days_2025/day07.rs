// https://adventofcode.com/2025/day/7

use std::collections::HashMap;
use crate::common::Solution;

fn update(m: &mut HashMap<usize,usize>, index: usize, n: usize) {
    if let Some(x) = m.get_mut(&index) {
        *x = *x + n
    } else {
        m.insert(index, n);
    }
}

pub fn solve(input: &str) -> Solution {
    let mut beams: HashMap<usize, usize> = HashMap::new();
    
    beams.insert(input.lines().next().unwrap().find('S').unwrap(), 1);

    let mut p1 = 0;
    for row in input.lines().skip(2).step_by(2) {
        let mut b: HashMap<usize, usize> = HashMap::new();
        for (i, n) in beams.iter() {
            if row.as_bytes()[*i] == b'^' {
                update(&mut b, i-1, *n);
                update(&mut b, i+1, *n);
                p1 = p1 + 1;
            } else {
                update(&mut b, *i, *n);
            }
        }
        beams = b;
    }
    let p2: usize = beams
        .iter()
        .map(|(_, v)| v)
        .sum::<usize>();

    Solution::new(p1, p2)
}

