// https://adventofcode.com/2024/day/16

use std::collections::HashSet;
use pathfinding::prelude::astar_bag_collect;
use crate::common::Solution;
use crate::grid::{Direction, Grid, GridElement, Position};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Maze {
    Wall,
    Empty,
    Start,
    End,
}

pub fn solve(input: &str) -> Solution {
    let grid: Grid<Maze> = Grid::load(input);

    let start: (Position, Direction) = (grid.find(&Maze::Start).unwrap(), Direction::East);
    let end = grid.find(&Maze::End).unwrap();

    let result = astar_bag_collect(
        &start,
        |(p, d)| {
            let mut suc: Vec<((Position, Direction), usize)> = Vec::new();
            let move_destination = p.along(&d);
            if grid.get(&move_destination).unwrap() != Maze::Wall {
                suc.push(((move_destination, d.clone()),1));
            }
            suc.push(((p.clone(), d.clockwise()), 1000));
            suc.push(((p.clone(), d.counter_clockwise()), 1000));
            suc.into_iter()
        },
        |(p,_)| end.column.abs_diff(p.column) + end.row.abs_diff(p.row) - 1,
        |(p,_)| grid.get(&p).unwrap() == Maze::End,
    )
    .unwrap();

    let p1 = result.1;
    let p2 = result
        .0
        .iter()
        .map(|v| v.iter())
        .flatten()
        .map(|e| e.0.clone())
        .collect::<HashSet<Position>>()
        .len();

    Solution::new(p1, p2)
}

impl GridElement for Maze {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '#' => Some(Maze::Wall),
            '.' => Some(Maze::Empty),
            'S' => Some(Maze::Start),
            'E' => Some(Maze::End),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Maze::Empty => '.',
            Maze::End => 'E',
            Maze::Start => 'S',
            Maze::Wall => '#',
        }
    }
}
