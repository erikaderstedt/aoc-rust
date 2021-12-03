// https://adventofcode.com/2021/day/3

use crate::common::Solution;
use crate::common::parsed_from_each_line;
use std::str::FromStr;

#[derive(Debug)]
struct Box {
    dimensions: [u32;3],
    longest: u32,
}

impl FromStr for Box {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<u32> = s.split('x').map(|v| v.parse::<u32>().unwrap()).collect();
        let longest = v.iter().max().unwrap().clone();
        Ok(Box { dimensions: [v[0], v[1], v[2]], longest })
    }
}

impl Box {
    fn wrapping_paper(&self) -> u32 {
        self.dimensions[0] * self.dimensions[1] * 2 +
        self.dimensions[1] * self.dimensions[2] * 2 +
        self.dimensions[2] * self.dimensions[0] * 2 + 
        self.dimensions[0] * self.dimensions[1] * self.dimensions[2] / self.longest
    }

    fn ribbon(&self) -> u32 {
        (self.dimensions[0] + self.dimensions[1] + self.dimensions[2] - self.longest) * 2 + 
        self.dimensions[0] * self.dimensions[1] * self.dimensions[2]
    }
}

pub fn solve(input: &str) -> Solution {
    let boxes: Vec<Box> = parsed_from_each_line(input);
    let m1 = boxes.iter().fold(0, |total, b| total + b.wrapping_paper());
    let m2 = boxes.iter().fold(0, |total, b| total + b.ribbon());
    Solution::new(m1,m2)
}