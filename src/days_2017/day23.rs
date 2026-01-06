// https://adventofcode.com/2017/day/23

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    // Part 1 - check if 84 is prime
    //          Each combination of [2..84], [2..84]
    //          is tried and requires a mul => answer is 82^2
    // Part 2 - check the number of composite numbers in interval,
    //          stepping by 17
    let b = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .skip(2)
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let p1 = (b - 2) * (b - 2);

    let b = b * 100 + 100000;
    let c = b + 17000;

    let p2 = (b..=c)
        .step_by(17)
        .filter(|&d| (2..d.isqrt()).any(|v| d % v == 0))
        .count();

    // let f = 1;
    // let d = 2;
    // let e = 2;
    // let g = d * e - b;
    // if d*e == b {
    //     f = 0;       // composite
    // }
    // let e = e + 1;

    // if f == 0,  increase h

    // if b != c {
    // b = b + 17;
    // }

    Solution::new(p1, p2)
}
