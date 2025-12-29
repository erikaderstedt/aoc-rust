// https://adventofcode.com/2018/day/11

use crate::common::Solution;

const W: usize = 300;

struct PowerLevel {
    at_x: usize,
    at_y: usize,
    level: i32,
}

impl PowerLevel {
    fn find_largest(grid: &Vec<Vec<i32>>, size: usize) -> PowerLevel {
        let mut best_x = 0;
        let mut best_y = 0;
        let mut best_value = i32::MIN;
        for x in size..=W {
            for y in size..=W {
                let power_level =
                    grid[y][x] - grid[y - size][x] - grid[y][x - size] + grid[y - size][x - size];
                if power_level > best_value {
                    best_value = power_level;
                    best_x = x;
                    best_y = y;
                }
            }
        }
        PowerLevel {
            at_x: best_x - size + 1,
            at_y: best_y - size + 1,
            level: best_value,
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let puzzle_input = input.trim().parse::<i32>().unwrap();
    let mut grid = vec![vec![0i32; W + 1]; W + 1];
    for x in 1..=W {
        for y in 1..=W {
            let rack_id = (x as i32) + 10;
            let value =
                (((rack_id * (y as i32) + puzzle_input) * rack_id) / 100).rem_euclid(10) - 5;
            grid[y][x] = value + grid[y - 1][x] + grid[y][x - 1] - grid[y - 1][x - 1];
        }
    }
    let largest_3x3 = PowerLevel::find_largest(&grid, 3);
    let largest = (1..=300)
        .map(|sz| (sz, PowerLevel::find_largest(&grid, sz)))
        .take_while(|v| v.1.level > 0) // Larger squares will have a lower and lower power level.
        .max_by_key(|m| m.1.level)
        .unwrap();

    let p1 = format!("{},{}", largest_3x3.at_x, largest_3x3.at_y);
    let p2 = format!("{},{},{}", largest.1.at_x, largest.1.at_y, largest.0);
    Solution::new(p1, p2)
}
