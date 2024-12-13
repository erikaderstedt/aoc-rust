// https://adventofcode.com/2024/day/13

use std::str::FromStr;
use itertools::Itertools;

use crate::common::Solution;

#[derive(Debug)]
struct Machine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
}

impl Machine {
    fn cost_to_win(self: &Machine, offset: isize) -> isize {
        let ax = self.button_a.0;
        let ay = self.button_a.1;
        let bx = self.button_b.0;
        let by = self.button_b.1;
        let zx = self.prize.0 + offset;
        let zy = self.prize.1 + offset;

        let pb = (zx*ay - zy*ax)/(bx*ay - ax*by);
        let pa = (zx - pb * bx)/ax;
        if pa * ax + pb * bx == zx && pa * ay + pb * by == zy { pa * 3 + pb } else { 0 }
    }
}

pub fn solve(input: &str) -> Solution {
 
    let machines: Vec<Machine> = input.split("\n\n").filter_map(|s| s.parse::<Machine>().ok()).collect();

    let p1: isize = machines.iter().map(|machine| machine.cost_to_win(0)).sum();
    let p2: isize = machines.iter().map(|machine| machine.cost_to_win(10000000000000)).sum();

    Solution::new(p1, p2)
}

impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l: Vec<&str> = s.lines().collect();
        let button_a = l[0].split(' ').skip(2).map(|v| v[2..4].parse::<isize>().unwrap()).collect_tuple().unwrap();
        let button_b = l[1].split(' ').skip(2).map(|v| v[2..4].parse::<isize>().unwrap()).collect_tuple().unwrap();
        let prize = l[2].split(' ').skip(1).map(|v| v[2..v.len()].split(',').next().unwrap().parse::<isize>().unwrap()).collect_tuple().unwrap();
        Ok( Machine { button_a, button_b, prize })
    }
}
