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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Reindeer {
    position: Position,
    direction: Direction,
}

pub fn solve(input: &str) -> Solution {
    let grid: Grid<Maze> = Grid::load(input);

    let start = Reindeer {
        position: grid.find(&Maze::Start).unwrap(),
        direction: Direction::East,
    };
    let end = grid.find(&Maze::End).unwrap();

    let result = astar_bag_collect(
        &start,
        |elf| {
            let mut suc: Vec<(Reindeer, usize)> = Vec::new();
            let move_destination = elf.position.along(&elf.direction);
            if grid.get(&move_destination).unwrap() != Maze::Wall {
                suc.push((
                    Reindeer {
                        position: move_destination,
                        direction: elf.direction.clone(),
                    },
                    1,
                ));
            }
            suc.push((
                Reindeer {
                    position: elf.position.clone(),
                    direction: elf.direction.clockwise(),
                },
                1000,
            ));
            suc.push((
                Reindeer {
                    position: elf.position.clone(),
                    direction: elf.direction.counter_clockwise(),
                },
                1000,
            ));
            suc.into_iter()
        },
        |elf| end.column.abs_diff(elf.position.column) + end.row.abs_diff(elf.position.row) - 1,
        |elf| grid.get(&elf.position).unwrap() == Maze::End,
    )
    .unwrap();

    let p1 = result.1;
    let p2 = result
        .0
        .iter()
        .map(|v| v.iter())
        .flatten()
        .map(|e| e.position.clone())
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
