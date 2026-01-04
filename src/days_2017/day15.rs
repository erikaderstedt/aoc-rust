// https://adventofcode.com/2017/day/15

use crate::common::Solution;
use itertools::Itertools;

struct Generator {
    value: usize,
    factor: usize,
}

impl Generator {
    fn starting_with(value: usize, factor: usize) -> Generator {
        Generator { value, factor }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // 2147483647 = 0x7fffffff
        // This is a Mersenne number (also a Mersenne prime)
        // Split the product in blocks of 31 bits, and add these blocks
        // If the sum is greater than 2^31, subtract 2^31 from the sum.
        let a = self.value * self.factor;
        let a = (a & 0x7fffffff) + (a >> 31);
        let v = if (a >> 31) != 0 { a - 0x7fffffff } else { a };
        self.value = v;
        // self.value = (self.value * self.factor) % 2147483647;
        Some(self.value)
    }
}

pub fn solve(input: &str) -> Solution {
    let (value_a, value_b) = input
        .lines()
        .map(|line| line.split(' ').last().unwrap().parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let p1 = Generator::starting_with(value_a, 16807)
        .zip(Generator::starting_with(value_b, 48271))
        .take(40_000_000)
        .filter(|(a, b)| (a & 0xffff) == (b & 0xffff))
        .count();
    let p2 = Generator::starting_with(value_a, 16807)
        .filter(|v| v & 3 == 0)
        .zip(Generator::starting_with(value_b, 48271).filter(|v| v & 7 == 0))
        .take(5_000_000)
        .filter(|(a, b)| (a & 0xffff) == (b & 0xffff))
        .count();
    Solution::new(p1, p2)
}
