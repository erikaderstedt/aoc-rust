// https://adventofcode.com/2021/day/3

use crate::common::Solution;

fn most_common_digit_in_position(numbers: &Vec<usize>, bit_position: usize) -> usize {
    if numbers.iter().fold(0, |t, n| t + ((n >> bit_position) & 1)) * 2 >= numbers.len() { 1 } else { 0 }
}

fn generic_rate_calculator(numbers: &Vec<usize>, num_digits: usize, a: usize) -> usize {
    (0..num_digits).fold(0, |value, bit_position| 
        value + if most_common_digit_in_position(&numbers, bit_position) == a { 1 << bit_position } else { 0 }
    )
}

fn generic_rating_calculator(numbers: &Vec<usize>, num_digits: usize, a: usize) -> usize {
    (0..num_digits).rev().fold(numbers.clone(), |l, bit_position| {
        if l.len() == 1 { l } else {
            let digit_to_retain = if most_common_digit_in_position(&l, bit_position) == a { 1 } else { 0 };
            l.into_iter().filter(|v| ((v >> bit_position) & 1) == digit_to_retain).collect()
        }
    })[0]
}

pub fn solve(input: &str) -> Solution {
    let numbers = input.lines().map(|line| usize::from_str_radix(line, 2).expect("Bad number")).collect();
    let num_digits = input.lines().next().unwrap().len();

    let gamma = generic_rate_calculator(&numbers, num_digits, 1);
    let epsilon = generic_rate_calculator(&numbers, num_digits, 0);
    let m1 = gamma * epsilon;

    let oxygen_generator_rating = generic_rating_calculator(&numbers, num_digits, 1);
    let co2_scrubber_rating = generic_rating_calculator(&numbers, num_digits, 0);
    let m2 = oxygen_generator_rating * co2_scrubber_rating;

    Solution::new(m1, m2)
}