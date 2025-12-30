// https://adventofcode.com/2018/day/22

use crate::{
    common::Solution,
    grid::{Grid, GridElement, Position},
};
use itertools::Itertools;
use pathfinding::prelude::astar;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tool {
    ClimbingGear,
    Torch,
    Neither,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cave {
    Rocky,
    Wet,
    Narrow,
}

// The shortest path may be slightly below or behind the target.
// Generate a bit extra. In my input 10 is enough.
const MARGIN: usize = 10;

pub fn solve(input: &str) -> Solution {
    let (depth, target) = input
        .lines()
        .map(|line| line.split(" ").skip(1).next().unwrap())
        .collect_tuple()
        .unwrap();

    let depth = depth.parse::<usize>().unwrap();
    let (target_x, target_y) = target
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    let height = target_y + 1 + MARGIN;
    let width = target_x + 1 + MARGIN;
    let mut erosion_levels = vec![0; width * height];
    let mut cave = vec![];
    let mut p1 = 0;
    for y in 0..height {
        for x in 0..width {
            let geologic_index = if (x == 0 && y == 0) || (x == target_x && y == target_y) {
                0
            } else if x == 0 {
                y * 48271
            } else if y == 0 {
                x * 16807
            } else {
                erosion_levels[y * width + x - 1] * erosion_levels[(y - 1) * width + x]
            };
            let erosion_level = (geologic_index + depth).rem_euclid(20183);
            erosion_levels[y * width + x] = erosion_level;
            let t = erosion_level.rem_euclid(3);
            cave.push(match t {
                0 => Cave::Rocky,
                1 => Cave::Wet,
                2 => Cave::Narrow,
                _ => panic!(""),
            });
            if x <= target_x && y <= target_y {
                p1 += t;
            }
        }
    }
    let cave: Grid<Cave> = Grid {
        rows: height,
        cols: width,
        locations: cave,
    };
    let start = (Position { row: 0, column: 0 }, Tool::Torch);
    let path = astar(
        &start,
        |(p, tool)| {
            let mut new_state: Vec<((Position, Tool), usize)> = vec![];
            for p2 in p.nearest_neighbors() {
                match cave.get(&p2) {
                    Some(Cave::Rocky) => {
                        if *tool == Tool::Neither {
                            new_state.push(((p2.clone(), Tool::ClimbingGear), 7 + 1));
                            new_state.push(((p2.clone(), Tool::Torch), 7 + 1));
                        } else {
                            new_state.push(((p2.clone(), tool.clone()), 1));
                        }
                    }
                    Some(Cave::Wet) => {
                        if *tool == Tool::Torch {
                            new_state.push(((p2.clone(), Tool::ClimbingGear), 7 + 1));
                            new_state.push(((p2.clone(), Tool::Neither), 7 + 1));
                        } else {
                            new_state.push(((p2.clone(), tool.clone()), 1));
                        }
                    }
                    Some(Cave::Narrow) => {
                        if *tool == Tool::ClimbingGear {
                            new_state.push(((p2.clone(), Tool::Neither), 7 + 1));
                            new_state.push(((p2.clone(), Tool::Torch), 7 + 1));
                        } else {
                            new_state.push(((p2.clone(), tool.clone()), 1));
                        }
                    }
                    _ => {}
                };
            }
            new_state.into_iter()
        },
        |(p, tool)| {
            let distance = (target_y - p.row) + (target_x - p.column);
            let tool_switch = if *tool != Tool::Torch { 7 } else { 0 };
            distance + tool_switch
        },
        |(p, tool)| p.row == target_y && p.column == target_x && *tool == Tool::Torch,
    );

    let p2 = path.unwrap().1;
    Solution::new(p1, p2)
}

impl GridElement for Cave {
    fn from_char(_c: &char) -> Option<Self> {
        panic!("not implemented");
    }
    fn to_char(&self) -> char {
        panic!("not implemented");
    }
}
