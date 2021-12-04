// https://adventofcode.com/2015/day/5

use crate::common::Solution;
use itertools::Itertools;

pub fn solve(input: &str) -> Solution {

    let m1 = input.lines()
        .filter(|s| s.chars().filter(|c| match c { 'a'|'e'|'i'|'o'|'u' => true, _ => false }).count() >= 3)
        .filter(|s| s.chars().tuple_windows().any(|(c1,c2)| c1 == c2))
        .filter(|s| !["xy","ab", "cd", "pq"].iter().any(|f| s.contains(f)))
        .count();
    // let m1 = input.lines()
    //     .filter(|s| s.chars().tuple_windows().any(|(c1,c2,c3)| c1 == c3 && c1 != c2))
    //     .filter(|s| s.chars().tuple_windows().enumerate()filter(|c| match c { 'a'|'e'|'i'|'o'|'u' => true, _ => false }).count() >= 3)
    //     .filter(|s| !forbidden_sequences.iter().any(|f| s.contains(f)))
    //     .count();

    Solution::new(m1, 0)
}