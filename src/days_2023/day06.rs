// https://adventofcode.com/2023/day/6

use crate::common::Solution;
const SURROUNDING: usize = 2;

fn number_of_wins(time: usize, distance: usize) -> usize {
    let approximate_start = (time - (((time*time - 4*distance) as f64).sqrt() as usize)) / 2;
    let approximate_end = (time + (((time*time - 4*distance) as f64).sqrt() as usize)) / 2;
    let around = |s: usize| -> std::ops::Range<usize> {
        s.checked_sub(SURROUNDING).unwrap_or(0)..(s + SURROUNDING + 1)
    };

    let start = around(approximate_start).find(|t| t * (time - t) > distance).unwrap();
    let end = around(approximate_end).rev().find(|t| t * (time - t) > distance).unwrap();
    
    end - start + 1
}

pub fn solve(input: &str) -> Solution {
    let times: Vec<usize> = input.split_ascii_whitespace()
        .filter_map(|s| s.parse::<usize>().ok()).take(4).collect();
    let distances: Vec<usize> = input.split_ascii_whitespace()
        .filter_map(|s| s.parse::<usize>().ok()).skip(4).collect();

    let p1: usize = times.into_iter().zip(distances.into_iter())
        .map(|(time,distance)| number_of_wins(time, distance))
        .product();

    let time: usize = input.lines().next().unwrap().split(":").last().unwrap()
        .replace(" ","").parse::<usize>().unwrap();
    let distance: usize = input.lines().skip(1).next().unwrap().split(":").last().unwrap()    
        .replace(" ","").parse::<usize>().unwrap();
    let p2 = number_of_wins(time, distance);

    Solution::new(p1,p2)
}
