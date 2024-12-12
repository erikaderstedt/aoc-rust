// https://adventofcode.com/2024/day/12

use crate::common::Solution;
use crate::grid::{Direction, Grid, Position};

fn explore_region(garden: &Grid<u8>, plant: &u8, from_location: &Position, visited: &mut Vec<Position>) -> (usize, usize) {
    visited.push(from_location.clone());

    let above = from_location.above();
    let below = from_location.below();
    let left = from_location.left();
    let right = from_location.right();

    let mut total_area = 1;
    let mut total_perimeter = 0;
    if garden.get(&above).unwrap() == *plant && !visited.contains(&above) {
        let r = explore_region(garden, plant, &above, visited);
        total_area += r.0;
        total_perimeter += r.1;
    }
    if garden.get(&below).unwrap() == *plant && !visited.contains(&below) {
        let r = explore_region(garden, plant, &below, visited);
        total_area += r.0;
        total_perimeter += r.1;
    }
    if garden.get(&left).unwrap() == *plant && !visited.contains(&left) {
        let r = explore_region(garden, plant, &left, visited);
        total_area += r.0;
        total_perimeter += r.1;
    }
    if garden.get(&right).unwrap() == *plant && !visited.contains(&right) {
        let r = explore_region(garden, plant, &right, visited);
        total_area += r.0;
        total_perimeter += r.1;
    }

    let perimeter = 4 - garden
        .neighbor_positions_satisfying_condition(&from_location, |_, other_plants| {
            other_plants == plant
        })
        .len();

    total_perimeter += perimeter;

    (total_area, total_perimeter)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Side {
    position: Position,
    facing: Direction,
}

fn explore_region_p2(garden: &Grid<u8>, plant: &u8, from_location: &Position, visited: &mut Vec<Position>, sides: &mut Vec<Side>) -> usize {
    visited.push(from_location.clone());

    let mut total_area = 1;

    {
        let above = from_location.above();
        if garden.get(&above).unwrap() == *plant {
            if !visited.contains(&above) {
                total_area += explore_region_p2(garden, plant, &above, visited, sides);
            }
        } else {
            sides.push(Side {
                position: from_location.clone(),
                facing: Direction::North,
            });
        }
    }
    {
        let below = from_location.below();
        if garden.get(&below).unwrap() == *plant {
            if !visited.contains(&below) {
                total_area += explore_region_p2(garden, plant, &below, visited, sides);
            }
        } else {
            sides.push(Side {
                position: from_location.clone(),
                facing: Direction::South,
            });
        }
    }

    {
        let left = from_location.left();
        if garden.get(&left).unwrap() == *plant {
            if !visited.contains(&left) {
                total_area += explore_region_p2(garden, plant, &left, visited, sides);
            }
        } else {
            sides.push(Side {
                position: from_location.clone(),
                facing: Direction::West,
            });
        }
    }
    {
        let right = from_location.right();

        if garden.get(&right).unwrap() == *plant {
            if !visited.contains(&right) {
                total_area += explore_region_p2(garden, plant, &right, visited, sides);
            }
        } else {
            sides.push(Side {
                position: from_location.clone(),
                facing: Direction::East,
            });
        }
    }

    total_area
}

fn explore_edges(edges: &Vec<Side>, facing: &Direction, position: &Position, visited: &mut Vec<Position>) {

    visited.push(position.clone());
    match facing {
        Direction::South | Direction::North => {
            if edges.contains(&Side { position: position.left(), facing: facing.clone() }) && !visited.contains(&position.left()) {
                explore_edges(edges, facing, &position.left(), visited);
            }
            if edges.contains(&Side {  position: position.right(), facing: facing.clone(), }) && !visited.contains(&position.right()) {
                explore_edges(edges, facing, &position.right(), visited);
            }
        }
        Direction::East | Direction::West => {
            if edges.contains(&Side { position: position.above(), facing: facing.clone(), }) && !visited.contains(&position.above()) {
                explore_edges(edges, facing, &position.above(), visited);
            }
            if edges.contains(&Side { position: position.below(), facing: facing.clone(), }) && !visited.contains(&position.below()) {
                explore_edges(edges, facing, &position.below(), visited);
            }
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let garden: Grid<u8> = Grid::load(input).enclosed(' ' as u8);

    let mut locations_to_visit: Vec<Position> = garden.positions().collect();

    let mut p1 = 0;
    while let Some(start) = locations_to_visit.pop() {
        let c = garden.get(&start).unwrap();
        if c == ' ' as u8 {
            continue;
        }

        let mut visited: Vec<Position> = Vec::new();
        let (area, perimeter) = explore_region(&garden, &c, &start, &mut visited);
        p1 += area * perimeter;
        for p in visited.into_iter().skip(1) {
            if let Some(i) = locations_to_visit.iter().position(|p1| *p1 == p) {
                locations_to_visit.remove(i);
            }
        }
    }

    let mut p2 = 0;

    locations_to_visit = garden.positions().collect();
    while let Some(start) = locations_to_visit.pop() {
        let c = garden.get(&start).unwrap();
        if c == ' ' as u8 {
            continue;
        }

        let mut visited: Vec<Position> = Vec::new();
        let mut edges: Vec<Side> = Vec::new();
        let area = explore_region_p2(&garden, &c, &start, &mut visited, &mut edges);
        // Count distinct sides
        let mut sides = 0;
        while let Some(edge) = edges.pop() {
            let mut visited: Vec<Position> = Vec::new();
            explore_edges(&edges, &edge.facing, &edge.position, &mut visited);
            for p in visited.into_iter().skip(1) {
                if let Some(i) = edges.iter().position(|p1| {
                    *p1 == Side {
                        position: p.clone(),
                        facing: edge.facing.clone(),
                    }
                }) {
                    edges.remove(i);
                }
            }
            sides += 1;
        }
        p2 += area * sides;
        for p in visited.into_iter().skip(1) {
            if let Some(i) = locations_to_visit.iter().position(|p1| *p1 == p) {
                locations_to_visit.remove(i);
            }
        }
    }

    Solution::new(p1, p2)
}
