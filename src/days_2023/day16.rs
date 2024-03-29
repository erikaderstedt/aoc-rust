// https://adventofcode.com/2023/day/16

use crate::{common::Solution, grid::{GridElement, Grid, Direction}};

#[derive(PartialEq, Eq, Clone, Debug)]
enum Stuff {
    Empty,
    MirrorForwardSlash,
    MirrorBackslash,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Beam {
    row: isize,
    column: isize,
    direction: Direction,
}

fn num_energized(grid: &Grid<Stuff>, beam: Beam) -> usize {
    let mut visited = vec![0u8; grid.cols * grid.rows];
    let mut beams: Vec<Beam> = vec![beam];

    while let Some(mut beam) = beams.pop() {

        loop {
            // Move beam
            let (row, column) = match beam.direction {
                Direction::East => { (beam.row, beam.column + 1) },
                Direction::North => { (beam.row - 1, beam.column) },
                Direction::West => { (beam.row, beam.column - 1) },
                Direction::South => { (beam.row + 1, beam.column )},
            };

            // Check out of bounds
            if row < 0 || row >= grid.rows as isize || column < 0 || column >= grid.cols as isize {
                break;
            }

            let p = (row as usize) * grid.cols + (column as usize);
                    
            if visited[p] & beam.direction.u8() != 0 {
                break
            } else {
                visited[p] |= beam.direction.u8();
            }
                    
            // Check what is there
            beam.row = row;
            beam.column = column;
            beam.direction = match grid.locations[p] {
                Stuff::Empty => { beam.direction },
                Stuff::HorizontalSplitter => {
                    match beam.direction {
                        Direction::East | Direction::West => { beam.direction },
                        Direction::South | Direction::North => {
                            beams.push(Beam { row, column, direction: Direction::East });
                            Direction::West
                        }
                    }
                },
                Stuff::VerticalSplitter => {
                    match beam.direction {
                        Direction::North | Direction::South => { beam.direction },
                        Direction::East | Direction::West => {
                            beams.push(Beam { row, column, direction: Direction::North} );
                            Direction::South
                        }
                    }
                },
                Stuff::MirrorBackslash => {
                    match beam.direction {
                        Direction::East => Direction::South,
                        Direction::West => Direction::North,
                        Direction::North => Direction::West,
                        Direction::South => Direction::East,
                    }
                },
                Stuff::MirrorForwardSlash => {
                    match beam.direction {
                        Direction::East => Direction::North,
                        Direction::West => Direction::South,
                        Direction::North => Direction::East,
                        Direction::South => Direction::West,
                    }
                }
            };
        }
    }

    visited.into_iter().filter(|u| *u != 0).count()
}

pub fn solve(input: &str) -> Solution {
    let grid : Grid<Stuff> = Grid::load(input);

    let p1 = num_energized(&grid, Beam { row: 0, column: -1, direction: Direction::East});

    let p2 =
    (1..grid.rows).map(|r| Beam { row: r as isize, column: -1, direction: Direction::East })
    .chain((0..grid.rows).map(|r| Beam { row: r as isize, column: grid.cols as isize, direction: Direction::West }))
    .chain((0..grid.cols).map(|c| Beam { row: -1, column: c as isize, direction: Direction::South }))
    .chain((0..grid.cols).map(|c| Beam { row: grid.rows as isize, column: c as isize, direction: Direction::North }))
    .map(|beam| num_energized(&grid, beam))
    .max().unwrap().max(p1);

    Solution::new(p1, p2)
}

impl GridElement for Stuff {
    fn from_char(c: &char) -> Option<Self> { 
        match c {
            '.' => Some(Self::Empty),
            '/' => Some(Self::MirrorForwardSlash),
            '\\' => Some(Self::MirrorBackslash),
            '-' => Some(Self::HorizontalSplitter),
            '|' => Some(Self::VerticalSplitter),
            _ => None,
        }
    }
    fn to_char(&self) -> char { 
        match self {
            Self::Empty => '.',
            Self::MirrorForwardSlash => '/',
            Self::MirrorBackslash => '\\',
            Self::HorizontalSplitter => '-',
            Self::VerticalSplitter => '|',
        }
    }
}