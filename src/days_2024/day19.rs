// https://adventofcode.com/2024/day/19

use pathfinding::prelude::{count_paths, dfs};

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let towel_patterns: Vec<&str> = input.split("\n\n").next().unwrap().split(", ").collect();
    let designs: Vec<&str> = input
        .split("\n\n")
        .skip(1)
        .next()
        .unwrap()
        .lines()
        .collect();

    let p1 = designs
        .iter()
        .filter(|design| -> bool {
            dfs(
                0usize,
                |i| {
                    let positions: Vec<usize> = towel_patterns
                        .iter()
                        .filter(|p| design[*i..design.len()].starts_with(**p))
                        .map(|p| i + p.len())
                        .collect();
                    positions
                },
                |s| *s == design.len(),
            )
            .is_some()
        })
        .count();

    let p2: usize = designs
    .iter()
    .map(|design| -> usize {
        count_paths(
            0usize,
            |i| {
                let positions: Vec<usize> = towel_patterns
                    .iter()
                    .filter(|p| design[*i..design.len()].starts_with(**p))
                    .map(|p| i + p.len())
                    .collect();
                positions
            },
            |s| *s == design.len(),
        )
    })
    .sum();

    Solution::new(p1, p2)
}
