// https://adventofcode.com/2024/day/15

use crate::{common::Solution, grid::{Direction, Grid, GridElement, Position}};

#[derive(Debug,Clone,PartialEq,Eq)]
enum Warehouse {
    Wall,
    Robot,
    Box,
    Empty,
}

pub fn solve(input: &str) -> Solution {

    let (warehouse_s, movements_s) = input.split_once("\n\n").unwrap();
    let mut warehouse: Grid<Warehouse> = Grid::load(warehouse_s);
    let movements: Vec<Direction> = movements_s.chars().filter_map(|c| -> Option<Direction> {
        match c {
        '^' => Some(Direction::North),
        '>' => Some(Direction::East),
        'v' => Some(Direction::South),
        '<' => Some(Direction::West),
        _ => None,
    }}).collect();

    let mut p: Position = warehouse.find(&Warehouse::Robot).expect("No robot found!");

    for m in movements.iter() {

        // There has to be a box
        // Search for empty, as long as no wall is found.
        // Move robots and boxes one step in that direction
        let step = warehouse
            .positions_along_direction(&p, m)
            .take_while(|p| warehouse.get(p) == Some(Warehouse::Box))
            .count() + 1;


        let c = match m {
            Direction::East => Position { row: p.row, column: p.column + step },
            Direction::South => Position { row: p.row + step, column: p.column },
            Direction::West => Position { row: p.row, column: p.column - step },
            Direction::North => Position { row: p.row - step, column: p.column },
        };

        if warehouse.get(&c) == Some(Warehouse::Wall) {
            continue;
        }

        assert!(warehouse.get(&c) == Some(Warehouse::Empty));
        let v: Vec<Position> = warehouse.positions_along_direction(&p, m).take(step).collect(); 
        for p1 in v.into_iter().rev() {
            let n = warehouse.get(&p1.along(&m.reverse())).unwrap();
            warehouse.set(&p1, n);
        }
        warehouse.set(&p, Warehouse::Empty);
        p = p.along(m);
    }
    // warehouse.display();

    let p1: usize = warehouse.positions().filter(|p| warehouse.get(p) == Some(Warehouse::Box))
    .map(|p| p.row * 100 + p.column).sum();

    let p2 = 0;

    Solution::new(p1, p2)
}


impl GridElement for Warehouse {
    fn from_char(c: &char) -> Option<Self> { 
        match c {
            '#' => Some(Self::Wall),
            '.' => Some(Self::Empty),
            'O' => Some(Self::Box),
            '@' => Some(Self::Robot),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Robot => '@',
            Self::Box => 'O',
            Self::Empty => '.',
            Self::Wall => '#',
        }
    }
}