// https://adventofcode.com/2025/day/4

use crate::common::Solution;
use crate::grid::{Grid, GridElement, Position};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Floor {
    Empty,
    Paper,
}

pub fn solve(input: &str) -> Solution {

    let mut grid = Grid::load(input);
    
    let removed: Vec<usize> = (0..grid.cols).map(|_| {
        let remove_these: Vec<Position> = grid.positions()
            .filter(|p| grid.get(p) == Some(Floor::Paper))
            .filter(|p| p.neighbors().filter(|neighbor| grid.get(neighbor) == Some(Floor::Paper)).count() < 4)
            .collect();
        for r in remove_these.iter() {
            grid.set(r, Floor::Empty);
        }
        remove_these.len()
    }).take_while(|&i| i > 0).collect();

    let p1 = removed[0];
    let p2 = removed.into_iter().sum::<usize>();
    
    Solution::new(p1, p2)
}

impl GridElement for Floor {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '@' => Some(Floor::Paper),
            '.' => Some(Floor::Empty),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Floor::Empty => '.',
            Floor::Paper => '@',
        }
    }
}
