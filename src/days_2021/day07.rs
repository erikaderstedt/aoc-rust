// https://adventofcode.com/2021/day/7
use crate::common::Solution;
use itertools::Itertools;

pub fn solve(input: &str) -> Solution {
    let numbers: Vec<isize> = input.split(',').map(|s| s.parse::<isize>().unwrap()).sorted().collect();

     // Provided that the probability distribution of X is such that the above expectation 
    // exists, then m is a median of X if and only if m is a minimizer of the mean absolute 
    // error with respect to X.[11] 
    // In particular, m is a sample median if and only if m minimizes the arithmetic mean 
    // of the absolute deviations.[12]
    let median = numbers[numbers.len()/2];
    let m1 = numbers.iter().fold(0isize, |s,&c| s + (c-median).abs());

    // If it is required to use a single number as a "typical" value for a set of known numbers 
    // x_{1},\dotsc ,x_{n}, then the arithmetic mean of the numbers does this best, in the sense
    // of minimizing the sum of squared deviations from the typical value: 
    // the sum of (x_{i}-{\bar {x}})^{2}. (It follows that the sample mean is also the best 
    // single predictor in the sense of having the lowest root mean squared error.)[2]
    // Note that this is not exactly the property we want to minimize. The difference is +- 0.5.
    // Check values above and below the mean.
    let mean = ((numbers.iter().sum::<isize>() as usize) / numbers.len()) as isize;
    let m2 = ((mean-1)..=(mean+1)).map(|k| 
        numbers.iter()
            .fold(0isize, |s,&c| {
                let diff = (c - k).abs();
                s + diff*(diff+1)/2
            })).min().unwrap();

    Solution::new(m1, m2)
}
