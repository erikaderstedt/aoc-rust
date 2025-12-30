// https://adventofcode.com/2018/day/11

use crate::{
    common::Solution,
    grid::{Grid, GridElement, Position},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Maze {
    Floor,
    Door(Direction),
    Wall,
    Start,
}

impl GridElement for Maze {
    fn from_char(_c: &char) -> Option<Self> {
        panic!("Not implemented");
    }

    fn to_char(&self) -> char {
        match self {
            Maze::Door(Direction::Horizontal) => '-',
            Maze::Door(Direction::Vertical) => '|',
            Maze::Floor => '.',
            Maze::Wall => '#',
            Maze::Start => 'X',
        }
    }
}

const SZ: usize = 204;

pub fn solve(input: &str) -> Solution {
    let mut maze = Grid {
        rows: SZ,
        cols: SZ,
        locations: vec![Maze::Wall; SZ * SZ],
    };
    let start = Position {
        row: SZ / 2,
        column: SZ / 2,
    };

    let mut p = start.clone();
    maze.set(&p, Maze::Start);
    let mut branch_positions = vec![];

    for c in input.chars() {
        match c {
            '^' => continue,
            'N' => {
                p.row = p.row - 1;
                maze.set(&p, Maze::Door(Direction::Horizontal));
                p.row = p.row - 1;
                maze.set(&p, Maze::Floor);
            }
            'S' => {
                p.row = p.row + 1;
                maze.set(&p, Maze::Door(Direction::Horizontal));
                p.row = p.row + 1;
                maze.set(&p, Maze::Floor);
            }
            'E' => {
                p.column = p.column + 1;
                maze.set(&p, Maze::Door(Direction::Vertical));
                p.column = p.column + 1;
                maze.set(&p, Maze::Floor);
            }
            'W' => {
                p.column = p.column - 1;
                maze.set(&p, Maze::Door(Direction::Vertical));
                p.column = p.column - 1;
                maze.set(&p, Maze::Floor);
            }
            '(' => {
                branch_positions.push(p.clone());
            }
            ')' => {
                p = branch_positions.pop().unwrap();
            }
            '|' => {
                p = branch_positions.last().unwrap().clone();
            }
            '$' => break,
            _ => panic!("Unknown character {}", c),
        }
    }

    // Flood fill starting at start.
    let mut fill = vec![0u16; maze.locations.len()];
    let start_index = start.row * maze.cols + start.column;
    fill[start_index] = 0;
    let mut positions_to_check: Vec<usize> = vec![start_index];
    while let Some(index) = positions_to_check.pop() {
        for neighbor in vec![index - 1, index + 1, index - maze.cols, index + maze.cols].into_iter()
        {
            match maze.locations[neighbor] {
                Maze::Start | Maze::Wall => continue,
                Maze::Floor if fill[neighbor] == 0 || fill[neighbor] > fill[index] => {
                    fill[neighbor] = fill[index];
                    positions_to_check.push(neighbor);
                }
                Maze::Door(_) if fill[neighbor] == 0 || fill[neighbor] > fill[index] + 1 => {
                    fill[neighbor] = fill[index] + 1;
                    positions_to_check.push(neighbor);
                }
                _ => continue,
            }
        }
    }
    let p1 = fill.iter().max().unwrap();
    let p2 = fill
        .iter()
        .enumerate()
        .filter(|(i, f)| maze.locations[*i] == Maze::Floor && **f >= 1000)
        .count();
    Solution::new(p1, p2)
}
