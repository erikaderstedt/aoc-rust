// https://adventofcode.com/2017/day/14

use crate::common::Solution;
use crate::days_2017::day10::knot_hash;

pub fn solve(input: &str) -> Solution {
    let mut grid = vec![0u8; 128 * 128];
    for row in 0..128 {
        let s = format!("{}-{}", input.trim(), row);
        for (i, h) in knot_hash(&s).iter().enumerate() {
            for k in 0..8 {
                grid[row * 128 + (i * 8 + 7 - k)] = (1 << k) & h;
            }
        }
    }
    let p1 = grid.iter().filter(|v| **v != 0).count();

    let mut p2 = 0;
    while let Some(non_zero_index) = grid.iter().position(|g| *g != 0) {
        grid[non_zero_index] = 0;
        let mut search = vec![non_zero_index];

        while let Some(idx) = search.pop() {
            // Check neighbors and add to search if != 0
            if idx >= 128 && grid[idx - 128] != 0 {
                grid[idx - 128] = 0;
                search.push(idx - 128);
            }
            if idx % 128 != 0 && grid[idx - 1] != 0 {
                grid[idx - 1] = 0;
                search.push(idx - 1);
            }
            if idx % 128 != 127 && grid[idx + 1] != 0 {
                grid[idx + 1] = 0;
                search.push(idx + 1);
            }
            if idx < grid.len() - 128 && grid[idx + 128] != 0 {
                grid[idx + 128] = 0;
                search.push(idx + 128);
            }
        }
        p2 += 1;
    }

    Solution::new(p1, p2)
}
