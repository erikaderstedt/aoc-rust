// https://adventofcode.com/2016/day/8

use itertools::Itertools;

use crate::{
    common::Solution,
    grid::{read_characters_off_grid, Grid},
};

pub fn solve(input: &str) -> Solution {
    let mut grid = Grid {
        rows: 6,
        cols: 50,
        locations: vec![false; 50 * 6],
    };

    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        if parts[0] == "rect" {
            let (wide, tall) = parts[1]
                .split("x")
                .map(|a| a.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            for i in 0..wide {
                for j in 0..tall {
                    grid.locations[j * grid.cols + i] = true;
                }
            }
        } else {
            let how_much = parts[4].parse::<usize>().unwrap();
            let index = parts[2][2..].parse::<usize>().unwrap();
            let c = grid.locations.clone();
            if parts[1] == "row" {
                for i in 0..grid.cols {
                    grid.locations[index * grid.cols + ((i + how_much) % grid.cols)] =
                        c[index * grid.cols + i];
                }
            } else {
                assert!(parts[1] == "column");
                for i in 0..grid.rows {
                    grid.locations[((i + how_much) % grid.rows) * grid.cols + index] =
                        c[i * grid.cols + index];
                }
            }
        }
    }
    let p1 = grid.locations.iter().filter(|d| **d).count();
    let p2 = read_characters_off_grid::<4, 6, 1>(&grid).unwrap();

    Solution::new(p1, p2)
}
