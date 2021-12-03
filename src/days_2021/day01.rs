// https://adventofcode.com/2021/day/1

use crate::common::Solution;
use crate::common::parsed_from_each_line;

const P1_STEP: usize = 1;
const P2_STEP: usize = 3;

pub fn solve(input: &str) -> Solution {
    let depths: Vec<usize> = parsed_from_each_line(input);
    let m1 = depths.windows(P1_STEP+1).filter(|w| w[P1_STEP] > w[0]).count();
    let m2 = depths.windows(P2_STEP+1).filter(|w| w[P2_STEP] > w[0]).count();
    Solution::new(m1, m2)
}