// https://adventofcode.com/2024/day/10

use crate::common::Solution;
use crate::grid::{Grid, Position};
use pathfinding::prelude::bfs_reach;

pub fn solve(input: &str) -> Solution {
    let topographic_map: Grid<u8> = Grid::load(input);

    let p1 = topographic_map
        .positions()
        .filter(|p| topographic_map.get(&p).unwrap() == 0)
        .map(|zero_location| {
            bfs_reach(zero_location, |s: &Position| {
                topographic_map
                    .neighbor_positions_satisfying_condition(&s, |this_height, next_height| {
                        *next_height == this_height + 1
                    })
                    .into_iter()
            })
            .filter(|p| topographic_map.get(p).unwrap() == 9)
            .count()
        })
        .sum::<usize>();

    let p2 = topographic_map
        .positions()
        .filter(|p| topographic_map.get(&p).unwrap() == 0)
        .map(|zero_location| {
            bfs_reach(
                (zero_location, vec![]),
                |s: &(Position, Vec<Position>)| {
                    let v: Vec<(Position, Vec<Position>)> = topographic_map
                        .neighbor_positions_satisfying_condition(
                            &s.0,
                            |this_height, next_height| *next_height == this_height + 1,
                        )
                        .into_iter()
                        .map(|p| (p, s.1.iter().chain([&s.0]).cloned().collect()))
                        .collect();
                    v.into_iter()
                },
            )
            .filter(|p| topographic_map.get(&p.0).unwrap() == 9)
            .count()
        })
        .sum::<usize>();

    Solution::new(p1, p2)
}
