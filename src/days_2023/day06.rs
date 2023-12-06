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

pub fn solve(_input: &str) -> Solution {
    let times = vec![47,70,75,66];
    let distances = vec![282,1079,1147,1062];
    let p1: usize = times.into_iter()
        .zip(distances.into_iter())
        .map(|(t, d)| number_of_wins(t, d)).product();

    let time: usize = 47707566;
    let distance: usize = 282107911471062;
    let p2 = number_of_wins(time, distance);

    Solution::new(p1,p2)
}
