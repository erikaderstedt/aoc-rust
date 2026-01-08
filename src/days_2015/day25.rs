// https://adventofcode.com/2015/day/25

use std::usize;

use itertools::Itertools;

use crate::common::Solution;

const FACTOR: usize = 252533;
const MODULO: usize = 33554393;
const VALUE: usize = 20151125;

pub fn solve(input: &str) -> Solution {
    let (row, column) = input
        .trim()
        .split(' ')
        .filter(|w| w.len() > 2 && w.chars().next().unwrap().is_ascii_digit())
        .map(|w| w[..(w.len() - 1)].parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    let side_of_full_triangle = row + column - 1;
    let elements_in_full_triangle = side_of_full_triangle * (side_of_full_triangle + 1) / 2;
    let n_steps = elements_in_full_triangle - row;

    // ((value * factor) mod m x factor) mod m
    // = ( (value * factor) mod m x (factor mod m)) mod m
    // = (value * factor * factor) mod m
    //  = ((value mod m) x (factor^2 mod m) mod m)
    // Calculate (factor ^ 2) mod m using modular exponentiation
    // (not 2, but n_steps of course)
    let p1 = (VALUE * mod_exp::mod_exp(FACTOR, n_steps, MODULO)) % MODULO;

    // (a x b) mod m = ((a mod m) x (b mod m)) mod m

    Solution::new(p1, 0)
}
