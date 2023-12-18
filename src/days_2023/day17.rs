// https://adventofcode.com/2023/day/17

use crate::{common::Solution, grid::{Grid, GridElement, Direction}};
use pathfinding::prelude::dijkstra;

#[derive(Debug,Clone,PartialEq, Eq, Hash)]
struct State {
    x: u8,
    y: u8,
    next_directions: [Direction;2],
}

impl State {
    fn proceed<const MIN_FOR_TURN: u8, const MAX_CONSECUTIVE: u8>(&self, grid: &Grid<u8>) -> Vec<(State,usize)> {
        let mut v: Vec<(State,usize)> = Vec::with_capacity(10);
        for direction in self.next_directions.iter() {
            let mut x = self.x as isize;
            let mut y = self.y as isize;
            let mut cost = 0;
            for i in 0..=MAX_CONSECUTIVE {
                if i >= MIN_FOR_TURN {
                    v.push((State { x: x as u8, y: y as u8, next_directions: direction.turns() }, cost as usize));            
                }
                match direction {
                    Direction::East => { x += 1; },
                    Direction::North => { y -= 1; },
                    Direction::West => { x -= 1; },
                    Direction::South => { y += 1; },
                };
                if x >= 0 && y >= 0 && x < grid.cols as isize && y < grid.rows as isize {
                    cost += grid.locations[(y as usize) * grid.cols + (x as usize)];
                } else {
                    break;
                }
            }
        }
        v
    }    
}

fn part<const MIN_FOR_TURN: u8, const MAX_CONSECUTIVE: u8>(grid: &Grid<u8>) -> usize {
    let is_end = |state: &State| -> bool {
        state.x as usize == grid.cols - 1 && state.y as usize == grid.rows - 1
    };

    let get_successors = |state: &State| { state.proceed::<MIN_FOR_TURN, MAX_CONSECUTIVE>(grid) };

    let start = State { x: 0, y: 0, next_directions: [Direction::East, Direction::South]};
    dijkstra(&start, get_successors, is_end).unwrap().1
}

pub fn solve(input: &str) -> Solution {
    let grid: Grid<u8> = Grid::load(input);

    let p1 = part::<1,3>(&grid);
    let p2 = part::<4,10>(&grid);

    Solution::new(p1, p2)
}

impl GridElement for u8 {
    fn from_char(c: &char) -> Option<Self> { 
        match c {
            '1'..='9' => Some((*c as u8) - ('0' as u8)),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            1 => '1',
            0 => '.',
            _ => '?',
        }
    }
}