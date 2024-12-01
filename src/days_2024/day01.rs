// https://adventofcode.com/2024/day/1

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let (mut a, mut b): (Vec<isize>, Vec<isize>) = input
        .lines()
        .flat_map(|line| -> Option<(isize, isize)> {
            match (line.split(" ").next(), line.split(" ").last()) {
                (Some(s1), Some(s2)) => {
                    Some((s1.parse::<isize>().unwrap(), s2.parse::<isize>().unwrap()))
                }
                _ => None,
            }
        })
        .unzip();

    a.sort_unstable();
    b.sort_unstable();

    let p1 = a
        .iter()
        .zip(b.iter())
        .map(|(v1, v2)| isize::abs(v1 - v2))
        .sum::<isize>();

    let p2 = a
        .iter()
        .map(|v1| (*v1) * b.iter().filter(|v2| v1 == *v2).count() as isize)
        .sum::<isize>();

    Solution::new(p1, p2)
}
