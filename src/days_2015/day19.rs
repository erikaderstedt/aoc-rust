// https://adventofcode.com/2015/day/19

use crate::common::Solution;
use std::collections::HashSet;

pub fn solve(input: &str) -> Solution {
    let (replacements, medicine) = input.split_once("\n\n").unwrap();
    let replacements: Vec<(&str, &str)> = replacements
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .collect();

    let medicine = medicine.to_string();
    let distinct_molecules: HashSet<String> = replacements
        .iter()
        .map(|(i, o)| {
            let strings: Vec<String> = medicine
                .match_indices(i)
                .map(|(index, _)| {
                    let mut m = medicine.clone();

                    m.replace_range(index..(index + i.len()), *o);
                    m
                })
                .collect();
            strings.into_iter()
        })
        .flatten()
        .collect();
    let p1 = distinct_molecules.len();

    // There are a number of elements that never converts to something else. In my input Rn, Y, Ar
    // Formulas are all one of these::
    // A => B Rn C Y D Y E Ar
    // A => B Rn C Y D Ar
    // A => B Rn C Ar
    // A => B C
    // If you have different tokens in your input, you should be able to inspect your
    // input to find the substitutes for Rn Y and Ar.

    let rn = medicine.matches("Rn").count();
    let y = medicine.matches("Y").count();
    let ar = medicine.matches("Ar").count();
    let total_steps = medicine.chars().filter(|c| c.is_ascii_uppercase()).count();

    // There are an equal number of rn, ar
    assert!(rn == ar);

    // Consider going backwards from the medicine molecule to e.
    // Any adjacent B C is converted to A -> 1 step
    // Each rn reduces the number of steps by 1, and each ar by 1.
    // Each y reduces the number of steps by 2.
    // 'e' is itself 1 long, so we are not reducing to zero, just to 1.
    let p2 = total_steps - rn - 2 * y - ar - 1;

    // It is not immediately clear (to me) that there exists a way between e and the molecule
    // but if it exists it must be this long.

    Solution::new(p1, p2)
}
