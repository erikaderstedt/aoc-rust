use crate::common::Solution;
use crate::common::parsed_from_each_line;

pub fn solve(input: &str) -> Solution {
    let depths: Vec<usize> = parsed_from_each_line(input);
    let m1 = depths.iter().skip(1).zip(depths.iter()).filter(|(s,f)| { s > f }).count();
    let m2 = depths.windows(3).skip(1).zip(depths.windows(3)).filter(|(s,f)| {
        s.iter().sum::<usize>() > f.iter().sum::<usize>()
    }).count();

    Solution::new(m1, m2)
}