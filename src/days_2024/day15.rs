// https://adventofcode.com/2024/day/15

use std::collections::HashSet;

use itertools::Itertools;

use crate::{common::Solution, grid::{Direction, Grid, GridElement, Position}};

#[derive(Debug,Clone,PartialEq,Eq)]
enum Warehouse {
    Wall,
    Robot,
    Box,
    Empty,
    BoxLeft,
    BoxRight,
}

fn normal_width(warehouse_s: &str, movements: &Vec<Direction>) -> usize {
    let mut warehouse: Grid<Warehouse> = Grid::load(warehouse_s);

    let mut p: Position = warehouse.find(&Warehouse::Robot).expect("No robot found!");

    for m in movements.iter() {
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

    warehouse
        .positions()
        .filter(|p| warehouse.get(p) == Some(Warehouse::Box))
        .map(|p| p.row * 100 + p.column).sum()
}

fn extra_width(warehouse_s: &str, movements: &Vec<Direction>) -> usize {
    let mut warehouse: Grid<Warehouse> = Grid::load(warehouse_s);
    let mut l: Vec<Warehouse> = vec![Warehouse::Empty; warehouse.locations.len() * 2];
    for i in 0..warehouse.locations.len() {
        match warehouse.locations[i] {
            Warehouse::Box => { l[i*2] = Warehouse::BoxLeft; l[i*2 + 1] = Warehouse::BoxRight; },
            Warehouse::Robot => { l[i*2] = Warehouse::Robot; },
            Warehouse::Wall => {  l[i*2] = Warehouse::Wall; l[i*2 +1] = Warehouse::Wall; }
            Warehouse::Empty | Warehouse::BoxLeft | Warehouse::BoxRight => {},
        };
    }
    warehouse.locations = l;
    warehouse.cols = warehouse.cols * 2;
    let mut p: Position = warehouse.find(&Warehouse::Robot).expect("No robot found!");

    for m in movements.iter() {

        if *m == Direction::North || *m == Direction::South {
            // For each box we need to include a "chain" of indices to move 
            let mut cols_to_check = vec![p.column];
            let mut row = p.row;
            let mut stuck = false;
            let mut can_move = false;
            let mut positions_to_shift = vec![p.clone()];
            while !stuck && !can_move {
                row = if *m == Direction::North { row - 1} else { row + 1};

                // If any cols to check find walls we skip to next movement
                // If all cols to check find empty we move
                // Otherwise add position to positions to shift
                let content: Vec<Warehouse> = cols_to_check.iter().map(|c| warehouse.locations[row * warehouse.cols + c].clone()).collect();
                if content.iter().contains(& Warehouse::Wall) {
                    stuck = true;
                    continue;
                } else if content.iter().all(|c| *c == Warehouse::Empty) {
                    can_move = true;
                    continue;
                }
                cols_to_check = cols_to_check.into_iter().map(|c| {
                    if warehouse.locations[row * warehouse.cols + c] == Warehouse::BoxLeft {
                        vec![c, c+1].into_iter()
                    } else if warehouse.locations[row*warehouse.cols + c] == Warehouse::BoxRight {
                        vec![c-1, c].into_iter()
                    } else {
                        vec![].into_iter()
                    }
                }).flatten().collect::<HashSet<usize>>().into_iter().collect();

                positions_to_shift.extend(cols_to_check.iter().map(|c| Position { row: row.clone(), column: c.clone() }));
            }
            if !stuck {
                for pq in positions_to_shift.into_iter().rev() {
                    let na = warehouse.get(&pq).unwrap();
                    let nb = warehouse.get(&pq.along(m)).unwrap();
                    warehouse.set(&pq, nb);
                    warehouse.set(&pq.along(m), na);
                }
                p = p.along(m);
            }
        } else {
            let step = warehouse
            .positions_along_direction(&p, m)
            .take_while(|p| warehouse.get(p) == Some(Warehouse::BoxLeft) || warehouse.get(p) == Some(Warehouse::BoxRight))
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

    }
    // warehouse.display();

    warehouse
    .positions()
    .filter(|p| warehouse.get(p) == Some(Warehouse::BoxLeft))
    .map(|p| p.row * 100 + p.column).sum()
}

pub fn solve(input: &str) -> Solution {

    let (warehouse_s, movements_s) = input.split_once("\n\n").unwrap();
    let movements: Vec<Direction> = movements_s.chars().filter_map(|c| -> Option<Direction> {
        match c {
        '^' => Some(Direction::North),
        '>' => Some(Direction::East),
        'v' => Some(Direction::South),
        '<' => Some(Direction::West),
        _ => None,
    }}).collect();

    let p1 = normal_width(warehouse_s, &movements);
    let p2 = extra_width(warehouse_s, &movements);

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
            Self::BoxLeft => '[',
            Self::BoxRight => ']',
        }
    }
}