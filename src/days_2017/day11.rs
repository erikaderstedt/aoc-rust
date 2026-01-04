// https://adventofcode.com/2017/day/11

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {

    let mut x:i64 = 0;
    let mut y:i64 = 0;
    let mut z:i64 = 0;

    let mut distances = vec![];
    for step in input.trim().split(',') {
        match step {
            "se" => { y -= 1; x += 1; },
            "nw" => { y += 1; x -= 1; },
            "sw" => { z += 1; x -= 1; },
            "ne" => { z -= 1; x += 1; },
            "n" => { y += 1; z -= 1; }
            "s" => { y -= 1; z += 1; }
            _ => panic!("Invalid step '{}'!", step),
        };
        distances.push((x.abs() + y.abs() + z.abs())/2);
    }
    let p1 = distances.last().unwrap();
    let p2 = distances.iter().max().unwrap();

    Solution::new(p1,p2)
}
