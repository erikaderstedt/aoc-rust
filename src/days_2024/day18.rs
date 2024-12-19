// https://adventofcode.com/2024/day/18

use itertools::Itertools;
use pathfinding::prelude::bfs;

use crate::{
    common::Solution,
    grid::{Grid, GridElement, Position},
};

const SZ: usize = 71;
const FIRST_PART: usize = 1024;

impl GridElement for bool {
    fn from_char(_: &char) -> Option<Self> {
        None
    }

    fn to_char(&self) -> char {
        match self {
            false => '.',
            true => '#',
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let mut corrupted: Grid<bool> = Grid {
        rows: SZ,
        cols: SZ,
        locations: vec![false; SZ * SZ],
    };
    let bytes: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    for (x, y) in bytes.iter().take(FIRST_PART) {
        corrupted.locations[y * SZ + x] = true;
    }
    let start = Position { row: 0, column: 0 };
    let end = Position {
        row: SZ - 1,
        column: SZ - 1,
    };

    let shortest_path = |grid: &Grid<bool>| -> Option<usize> {
        bfs(
            &start,
            |position| grid.neighbor_positions_satisfying_condition(position, |_, n| !n),
            |position| *position == end,
        )
        .map(|v| v.len())
    };

    let p1 = shortest_path(&corrupted).unwrap();

    // Do an initial binary search between bytes.len() - 1024
    let mut pieces = 2;
    let interval = bytes.len() - FIRST_PART;
    let mut current = interval / pieces;
    loop {
        let mut grid = corrupted.clone();
        pieces *= 2;
        let step = interval / pieces;
        for (x, y) in bytes.iter().skip(FIRST_PART).take(current) {
            grid.locations[y * SZ + x] = true;
        }
        if shortest_path(&grid) == None {
            current = current - step
        } else {
            current = current + step
        }
        if step <= 4 {
            break;
        }
    }

    let base = current - 4;
    for (x, y) in bytes.iter().skip(FIRST_PART).take(base) {
        corrupted.locations[y * SZ + x] = true;
    }

    let mut p2: String = "".to_string();
    for (x, y) in bytes.iter().skip(FIRST_PART + base) {
        corrupted.locations[y * SZ + x] = true;
        if shortest_path(&corrupted) == None {
            p2 = format!("{},{}", x, y);
            break;
        }
    }

    Solution::new(p1, p2)
}
