// https://adventofcode.com/2022/day/2

use crate::common::Solution;

const ROCK: usize = 1;
const PAPER: usize = 2;
const SCISSORS: usize = 3;

const WIN: usize = 6;
const TIE: usize = 3;
const LOSS: usize = 0;

pub fn solve(input: &str) -> Solution {
    let (p1,p2) = input
        .split("\n")
        .flat_map(|line| {
            match line {
                "A X" => Some((ROCK + TIE,      SCISSORS + LOSS)),
                "A Y" => Some((PAPER + WIN,     ROCK + TIE)),
                "A Z" => Some((SCISSORS + LOSS, PAPER + WIN)),
                "B X" => Some((ROCK + LOSS,     ROCK + LOSS)),
                "B Y" => Some((PAPER + TIE,     PAPER + TIE)),
                "B Z" => Some((SCISSORS + WIN,  SCISSORS + WIN)),
                "C X" => Some((ROCK + WIN,      PAPER + LOSS)),
                "C Y" => Some((PAPER + LOSS,    SCISSORS + TIE)),
                "C Z" => Some((SCISSORS + TIE,  ROCK + WIN)),
                _ => None
            }})
        .fold((0,0), |sum, scores| (sum.0 + scores.0, sum.1 + scores.1));    

    Solution::new(p1, p2)
}
