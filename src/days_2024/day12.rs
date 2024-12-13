// https://adventofcode.com/2024/day/12

use crate::common::Solution;
use crate::grid::{Direction, Grid, Position};

fn explore_region(garden: &Grid<u8>, plant: &u8, from_location: &Position, visited: &mut Vec<Position>) -> (usize, usize) {
    visited.push(from_location.clone());

    let (sub_area, sub_perimeter) = vec![Direction::East, Direction::North, Direction::South, Direction::West]
        .into_iter()
        .fold((0,0), |(area, perimeter), direction| -> (usize, usize) {
            let p = from_location.along(&direction);
            if garden.get(&p) == Some(*plant) && !visited.contains(&p) {
                let (t_area, t_perimeter) = explore_region(garden, plant, &p, visited);
                (area + t_area, perimeter + t_perimeter)
            } else {
                (area, perimeter)
            }
        });

    let valid_neighbors = garden
    .neighbor_positions_satisfying_condition(&from_location, |_, other_plants| {
        other_plants == plant
    });

    let perimeter = 4 - valid_neighbors.len();

    (sub_area + 1, perimeter + sub_perimeter)
}

pub fn solve(input: &str) -> Solution {
    let garden: Grid<u8> = Grid::load(input).enclosed(' ' as u8);

    let mut locations_to_visit: Vec<Position> = garden.positions().collect();

    let mut p1 = 0;
    let mut p2 = 0;
    while let Some(start) = locations_to_visit.pop() {
        let c = garden.get(&start).unwrap();
        if c == ' ' as u8 {
            continue;
        }

        let mut visited: Vec<Position> = Vec::new();
        let (area, perimeter) = explore_region(&garden, &c, &start, &mut visited);
        // Count number of corners in visited
        let mut corners = 0;
        for v in visited.iter() {
            // Outside corners
            if !visited.contains(&v.left()) && !visited.contains(&v.above()) { corners += 1; }
            if !visited.contains(&v.right()) && !visited.contains(&v.above()) { corners += 1; }
            if !visited.contains(&v.right()) && !visited.contains(&v.below()) { corners += 1; }
            if !visited.contains(&v.left()) && !visited.contains(&v.below()) { corners += 1; }
            // Inside corners
            if !visited.contains(&v.left()) && visited.contains(&v.above_left()) && visited.contains(&v.above()) { corners += 1; }
            if !visited.contains(&v.above()) && visited.contains(&v.above_right()) && visited.contains(&v.right()) { corners += 1; }
            if !visited.contains(&v.right()) && visited.contains(&v.below_right()) && visited.contains(&v.below()) { corners += 1; }
            if !visited.contains(&v.below()) && visited.contains(&v.below_left()) && visited.contains(&v.left()) { corners += 1; }
        }
        
        p1 += area * perimeter;
        p2 += area * corners;
        for p in visited.into_iter().skip(1) {
            if let Some(i) = locations_to_visit.iter().position(|p1| *p1 == p) {
                locations_to_visit.remove(i);
            }
        }
    }

    Solution::new(p1, p2)
}
