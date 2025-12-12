// https://adventofcode.com/2025/day/12

use itertools::Itertools;
use crate::common::Solution;

struct Region {
    width: usize,
    height: usize,
    gifts: [usize;6]
}

pub fn solve(input: &str) -> Solution {

    let regions: Vec<Region> = input.lines().filter(|line| line.contains('x'))
        .map(|line| {
            let gifts = line.split(' ').skip(1).map(|p| p.parse::<usize>().unwrap()).collect_array().unwrap();
            let (width, height) = line.split(':').take(1).next().unwrap().split('x').map(|v| v.parse::<usize>().unwrap()).collect_tuple().unwrap();
            Region { width, height, gifts }
        }).collect();

    let fills: Vec<usize> = input.split("\n\n")
        .map(|s| s.chars().filter(|&c| c == '#').count()).collect();

    let p1: usize = regions.iter()
        .filter(|r| 
            r.gifts.iter().zip(fills.iter()).map(|(a,b)| a * b).sum::<usize>() < r.width * r.height
        )
        .count();

    let p2 = 0;
    
    Solution::new(p1, p2)
}
