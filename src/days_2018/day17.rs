// https://adventofcode.com/2018/day/17

use crate::common::Solution;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum WallDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Ground {
    Sand,
    Clay,
    FlowingWater,
    PooledWater,
}

const SPRING: usize = 500;
const SURFACE: usize = 0;

pub fn solve(input: &str) -> Solution {
    // Old implementation that I did when learning Rust. I've cleaned it up a bit.
    //
    let walls: Vec<(WallDirection, usize, usize, usize)> = input
        .lines()
        .map(|line| {
            let (d1, d2) = line.split_once(", ").unwrap();
            let d = if d1.as_bytes()[0] == 'x' as u8 {
                WallDirection::Vertical
            } else {
                WallDirection::Horizontal
            };
            let v1 = d1.split('=').last().unwrap().parse::<usize>().unwrap();
            let (v2, v3) = d2
                .split('=')
                .last()
                .unwrap()
                .split("..")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            (d, v1, v2, v3)
        })
        .collect();

    let vertical_walls: Vec<(usize, usize, usize)> = walls
        .iter()
        .filter(|w| w.0 == WallDirection::Vertical)
        .map(|(_, a, b, c)| (*a, *b, *c))
        .collect();
    let horizontal_walls: Vec<(usize, usize, usize)> = walls
        .iter()
        .filter(|w| w.0 == WallDirection::Horizontal)
        .map(|(_, a, b, c)| (*a, *b, *c))
        .collect();

    let min_x = vertical_walls
        .iter()
        .map(|w| w.0)
        .chain(horizontal_walls.iter().map(|w| w.1))
        .min()
        .unwrap_or(0);
    let max_x = vertical_walls
        .iter()
        .map(|w| w.0)
        .chain(horizontal_walls.iter().map(|w| w.2))
        .max()
        .unwrap_or(0);
    let min_y = horizontal_walls
        .iter()
        .map(|w| w.0)
        .chain(vertical_walls.iter().map(|w| w.1))
        .min()
        .unwrap_or(0);
    let max_y = horizontal_walls
        .iter()
        .map(|w| w.0)
        .chain(vertical_walls.iter().map(|w| w.2))
        .max()
        .unwrap_or(0);

    let width: usize = max_x - min_x + 8;
    let offset: usize = min_x - 4;
    let mut grid: Vec<Vec<Ground>> = vec![vec![Ground::Sand; width]; max_y + 4];

    for (x, y1, y2) in vertical_walls {
        for y in y1..=y2 {
            grid[y][x - offset] = Ground::Clay;
        }
    }
    for (y, x1, x2) in horizontal_walls {
        for x in x1..=x2 {
            grid[y][x - offset] = Ground::Clay;
        }
    }

    // 0 - empty
    // 1 - clay
    // 2 - flowing water
    // 3 - pool
    let mut water_downstreams: Vec<(usize, usize)> = vec![(SPRING - offset, SURFACE)];
    while let Some(downstream) = water_downstreams.pop() {
        let mut y = downstream.1;
        let x = downstream.0;

        while y < max_y && grid[y + 1][x] == Ground::Sand {
            grid[y + 1][x] = Ground::FlowingWater;
            y += 1;
        }

        if y == max_y || grid[y + 1][x] == Ground::FlowingWater {
            continue;
        } // Reached end or overflowing surface of water.

        let mut new_spill_points: Vec<(usize, usize)> = vec![];

        while new_spill_points.len() == 0 {
            let mut x0 = x;
            while (grid[y + 1][x0] == Ground::Clay || grid[y + 1][x0] == Ground::PooledWater)
                && grid[y][x0] != Ground::Clay
            {
                grid[y][x0] = Ground::FlowingWater;
                x0 += 1;
            }
            if grid[y][x0] != Ground::Clay {
                // Spilling on the right
                grid[y][x0] = Ground::FlowingWater;
                new_spill_points.push((x0, y));
            }

            let mut x1 = x;
            while (grid[y + 1][x1] == Ground::Clay || grid[y + 1][x1] == Ground::PooledWater)
                && grid[y][x1] != Ground::Clay
            {
                grid[y][x1] = Ground::FlowingWater;
                x1 -= 1;
            }
            if grid[y][x1] != Ground::Clay {
                // Spilling on the left
                grid[y][x1] = Ground::FlowingWater;
                new_spill_points.push((x1, y));
            }

            if new_spill_points.len() == 0 {
                // Fill one layer of the pool
                for i in (x1 + 1)..x0 {
                    grid[y][i] = Ground::PooledWater;
                }
                y -= 1;
            }
        }
        water_downstreams.extend(new_spill_points);
    }

    let flowing_water: usize = grid
        .clone()
        .into_iter()
        .skip(min_y)
        .map(|x| x.into_iter().filter(|y| *y == Ground::FlowingWater).count())
        .sum();
    let pooled_water: usize = grid
        .into_iter()
        .map(|x| {
            x.into_iter()
                .skip(min_y)
                .filter(|y| *y == Ground::PooledWater)
                .count()
        })
        .sum();
    let p1 = flowing_water + pooled_water;
    let p2 = pooled_water;
    Solution::new(p1, p2)
}
