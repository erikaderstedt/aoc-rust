use crate::common::Solution;
use crate::common::parsed_from_each_line;
use std::collections::HashSet;

pub fn solve(input: &str) -> Solution {
    let frequency_changes: Vec<i64> = parsed_from_each_line(input);
    let p1 = frequency_changes.iter().sum::<i64>();
    let mut frequencies: HashSet<i64> = HashSet::new();
    let mut frequency = 0;

    for change in frequency_changes.iter().cycle() {
        if !frequencies.insert(frequency) { break }
        frequency += change;
    }

    Solution::new(p1,frequency)
}