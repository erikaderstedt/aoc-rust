// https://adventofcode.com/2024/day/15

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

fn extra_width_warehouse(warehouse: &Grid<Warehouse>) -> Grid<Warehouse> {
    let mut l: Vec<Warehouse> = vec![Warehouse::Empty; warehouse.locations.len() * 2];
    for i in 0..warehouse.locations.len() {
        match warehouse.locations[i] {
            Warehouse::Box => { l[i*2] = Warehouse::BoxLeft; l[i*2 + 1] = Warehouse::BoxRight; },
            Warehouse::Robot => { l[i*2] = Warehouse::Robot; },
            Warehouse::Wall => {  l[i*2] = Warehouse::Wall; l[i*2 +1] = Warehouse::Wall; }
            Warehouse::Empty | Warehouse::BoxLeft | Warehouse::BoxRight => {},
        };
    }
    Grid { rows: warehouse.rows, cols: warehouse.cols * 2, locations: l }
}

fn follow_movements<const PUSH_MULTIPLE: bool>(warehouse: &mut Grid<Warehouse>, movements: &Vec<Direction>) -> usize {
    let mut p: Position = warehouse.find(&Warehouse::Robot).expect("No robot found!");
    warehouse.set(&p, Warehouse::Empty);
    
    for m in movements.iter() {
        if PUSH_MULTIPLE && (*m == Direction::North || *m == Direction::South) {
            let mut cols_to_check = vec![p.column];
            let mut row = p.row;
            let mut positions_to_shift = vec![];
            loop {
                row = if *m == Direction::North { row - 1} else { row + 1};
                let mut next_cols_to_check: Vec<usize> = Vec::with_capacity(cols_to_check.len() * 2);
                let mut found_wall = false;
                for c in cols_to_check.into_iter() {
                    match warehouse.locations[row * warehouse.cols + c] {
                        Warehouse::BoxLeft => { next_cols_to_check.push(c); next_cols_to_check.push(c + 1); },
                        Warehouse::BoxRight => { next_cols_to_check.push(c-1); next_cols_to_check.push(c); },
                        Warehouse::Box => { next_cols_to_check.push(c); },
                        Warehouse::Empty => {},
                        Warehouse::Robot => { panic!("Should not find the robot while doing this."); }
                        Warehouse::Wall => { found_wall = true; }
                    }
                }
                if found_wall {
                    break;
                } else if next_cols_to_check.len() == 0 {
                    for pq in positions_to_shift.iter().rev() {
                        let na = warehouse.get(pq).unwrap();
                        let nb = warehouse.get(&pq.along(m)).unwrap();
                        warehouse.set(pq, nb);
                        warehouse.set(&pq.along(m), na);
                    }
                    p = p.along(m);
                    break;
                } else {
                    next_cols_to_check.sort_unstable();
                    next_cols_to_check.dedup();
                    cols_to_check = next_cols_to_check;
                    positions_to_shift.extend(cols_to_check.iter().map(|c| Position { row: row.clone(), column: c.clone() }));
                }
            }
        } else {
            let boxes_in_a_row = warehouse
                .positions_along_direction(&p, m)
                .take_while(|p| match warehouse.get(p).unwrap() {
                    Warehouse::BoxLeft | Warehouse::BoxRight | Warehouse::Box => true,
                _ => false, })
                .count();

            let last_box_position = match m {
                Direction::East => Position { row: p.row, column: p.column + boxes_in_a_row },
                Direction::South => Position { row: p.row + boxes_in_a_row, column: p.column },
                Direction::West => Position { row: p.row, column: p.column - boxes_in_a_row },
                Direction::North => Position { row: p.row - boxes_in_a_row, column: p.column },
            };

            if warehouse.get(&last_box_position.along(m)) == Some(Warehouse::Empty) {
                let box_contents: Vec<Warehouse> = warehouse
                .positions_along_direction(&p, m)
                .take(boxes_in_a_row)
                .map(|p| warehouse.get(&p).unwrap())
                .collect(); 
        
                for (new_box_position, content) in warehouse
                    .positions_along_direction(&p, m)
                    .skip(1)
                    .zip(box_contents.into_iter()) {
                    warehouse.set(&new_box_position, content);
                }
                
                p = p.along(m);
                warehouse.set(&p, Warehouse::Empty);
            }
        }
    }

    warehouse
        .positions()
        .filter(|p| warehouse.get(p) == Some(Warehouse::BoxLeft) || warehouse.get(p) == Some(Warehouse::Box))
        .map(|p| p.row * 100 + p.column).sum()
}

pub fn solve(input: &str) -> Solution {
    let (warehouse_s, movements_s) = input.split_once("\n\n").unwrap();

    let mut warehouse: Grid<Warehouse> = Grid::load(warehouse_s);
    let mut extra_wide_warehouse = extra_width_warehouse(&warehouse);
    let movements: Vec<Direction> = movements_s.chars().filter_map(|c| -> Option<Direction> {
        match c {
        '^' => Some(Direction::North),
        '>' => Some(Direction::East),
        'v' => Some(Direction::South),
        '<' => Some(Direction::West),
        _ => None,
    }}).collect();

    let p1 = follow_movements::<false>(&mut warehouse, &movements);
    let p2 = follow_movements::<true>(&mut extra_wide_warehouse, &movements);

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