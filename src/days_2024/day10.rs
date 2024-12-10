// https://adventofcode.com/2024/day/10

use crate::common::Solution;
use crate::grid::{Grid, Position};
use pathfinding::prelude::bfs_reach;

pub fn solve(input: &str) -> Solution {
    let topographic_map: Grid<u8> = Grid::load(input).enclosed('X' as u8);

    let p1 = topographic_map
        .locations
        .iter()
        .enumerate()
        .filter(|(_, i)| **i == 0)
        .map(|(index, _)| {
            let zero_location = Position {
                row: index / topographic_map.cols,
                column: index % topographic_map.cols,
            };
            let j = bfs_reach((zero_location.clone(), 0u8), |s: &(Position, u8)| {
                vec![
                    (s.0.above(), s.1 + 1),
                    (s.0.below(), s.1 + 1),
                    (s.0.left(), s.1 + 1),
                    (s.0.right(), s.1 + 1),
                ]
                .into_iter()
                .filter(|(p1, n)| topographic_map.get(p1).unwrap() == *n)
            })
            .filter(|p| p.1 == 9u8)
            // .inspect(|(p, n)| println!("From {:?} we reach {:?} (a {})", zero_location, p, n))
            .count();
            j
        })
        .sum::<usize>();

    let p2 = topographic_map
        .locations
        .iter()
        .enumerate()
        .filter(|(_, i)| **i == 0)
        .map(|(index, _)| {
            let zero_location = Position {
                row: index / topographic_map.cols,
                column: index % topographic_map.cols,
            };
            let j = bfs_reach(
                (zero_location.clone(), 0u8, vec![]),
                |s: &(Position, u8, Vec<Position>)| {
                    vec![
                        (s.0.above(), s.1 + 1, { let mut v: Vec<Position> = s.2.iter().cloned().collect(); v.push(s.0.clone()); v } ),
                        (s.0.below(), s.1 + 1, { let mut v: Vec<Position> = s.2.iter().cloned().collect(); v.push(s.0.clone()); v } ),
                        (s.0.left(), s.1 + 1, { let mut v: Vec<Position> = s.2.iter().cloned().collect(); v.push(s.0.clone()); v } ),
                        (s.0.right(), s.1 + 1, { let mut v: Vec<Position> = s.2.iter().cloned().collect(); v.push(s.0.clone()); v} )
                    ]
                    .into_iter()
                    .filter(|(p1, n, _)| topographic_map.get(p1).unwrap() == *n)
                },
            )
            .filter(|p| p.1 == 9u8)
            .count();
            j
        })
        .sum::<usize>();

    Solution::new(p1, p2)
}
