// https://adventofcode.com/2023/day/17

use crate::{common::Solution, grid::{Grid, GridElement, Direction}};
use pathfinding::prelude::dijkstra;

#[derive(Debug,Clone,PartialEq, Eq, Hash)]
struct State {
    x: u8,
    y: u8,
    consecutive: u8,
    direction: Direction,
}

impl State {
    fn step(&self, direction: Direction, grid: &Grid<u8>) -> Option<(State,usize)> {
        let mut x = self.x as isize;
        let mut y = self.y as isize;
        match direction {
            Direction::East => { x += 1; },
            Direction::North => { y -= 1; },
            Direction::West => { x -= 1; },
            Direction::South => { y += 1; },
        };
        if x >= 0 && y >= 0 && x < grid.cols as isize && y < grid.rows as isize {
            let cost = grid.locations[(y as usize) * grid.cols + (x as usize)] as usize;
            let consecutive = if direction == self.direction { self.consecutive + 1 } else { 1 };
            Some((State { x: x as u8, y: y as u8, consecutive , direction }, cost))
        } else {
            None
        }
    }    
}

pub fn solve(input: &str) -> Solution {
    let grid: Grid<u8> = Grid::load(input);

    let is_end = |state: &State| -> bool {
        state.x as usize == grid.cols - 1 && state.y as usize == grid.rows - 1
    };
    
    let get_successors_p1 = |state: &State| -> Vec<(State,usize)> {
        let mut v: Vec<(State,usize)> = Vec::with_capacity(3);
        if state.consecutive < 3 { if let Some(x) = state.step(state.direction.clone(), &grid) { v.push(x); } }
        if let Some(x) = state.step(state.direction.clockwise(), &grid) { v.push(x); }
        if let Some(x) = state.step(state.direction.counter_clockwise(), &grid) { v.push(x); }
        v
    };

    let get_successors_p2 = |state: &State| -> Vec<(State,usize)> {
        let mut v: Vec<(State,usize)> = Vec::with_capacity(3);
        if state.consecutive < 10 { if let Some(x) = state.step(state.direction.clone(), &grid) { v.push(x); } }
        if state.consecutive >= 4 { if let Some(x) = state.step(state.direction.clockwise(), &grid) { v.push(x); } }
        if state.consecutive >= 4 { if let Some(x) = state.step(state.direction.counter_clockwise(), &grid) { v.push(x); } }
        v
    };

    let start = State { x: 0, y: 0, consecutive: 0, direction: Direction::East};
    let p1 = dijkstra(&start, get_successors_p1, is_end).unwrap().1;
    let p2 = dijkstra(&start, get_successors_p2, is_end).unwrap().1;
    
    Solution::new(p1, p2)
}

impl GridElement for u8 {
    fn from_char(c: &char) -> Option<Self> { 
        match c {
            '1'..='9' => Some((*c as u8) - ('0' as u8)),
            _ => None,
        }
    }
    fn to_char(&self) -> char { '.' }
}