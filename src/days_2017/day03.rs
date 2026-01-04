// https://adventofcode.com/2017/day/3

use crate::common::Solution;

fn distance_to_value(value: &u64) -> u64 {
    let mut level = 1;
    let mut sum = 1;
    loop {
        for _ in 0..2 {
            if sum + level > *value {
                let d = *value - sum;
                return if d < level / 2 { level - d } else { d };
            }
            sum += level;
        }
        level += 1;
    }
}

struct SpiralIterator {
    grid: Vec<Vec<u64>>,
    x: usize,
    y: usize,
    level: u64,
    step: u64,
    direction: usize,
}

const DIRECTIONS: [[isize; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];
const SZ: usize = 40;

impl SpiralIterator {
    fn new() -> SpiralIterator {
        let mut grid = vec![vec![0; SZ]; SZ];
        let x = SZ / 2;
        let y = SZ / 2;
        grid[x][y] = 1;
        SpiralIterator {
            grid,
            x,
            y,
            level: 1,
            step: 0,
            direction: 0,
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.step += 1;
        self.x = ((self.x as isize) + DIRECTIONS[self.direction][0]) as usize;
        self.y = ((self.y as isize) + DIRECTIONS[self.direction][1]) as usize;
        if self.step == self.level {
            self.step = 0;
            self.direction += 1;
            if self.direction & 1 == 0 {
                self.level += 1;
            }
            if self.direction == 4 {
                self.direction = 0;
            }
        }
        let value = self.grid[self.x - 1][self.y]
            + self.grid[self.x + 1][self.y]
            + self.grid[self.x - 1][self.y - 1]
            + self.grid[self.x][self.y - 1]
            + self.grid[self.x + 1][self.y - 1]
            + self.grid[self.x - 1][self.y + 1]
            + self.grid[self.x][self.y + 1]
            + self.grid[self.x + 1][self.y + 1];
        self.grid[self.x][self.y] = value;
        Some(value)
    }
}

pub fn solve(input: &str) -> Solution {
    let v = input.trim().parse::<u64>().unwrap();

    let p1 = distance_to_value(&v);
    let p2 = SpiralIterator::new()
        .skip_while(|value| *value < v)
        .next()
        .unwrap();

    Solution::new(p1, p2)
}
