// https://adventofcode.com/2023/day/21

use crate::{common::Solution, grid::{Grid, GridElement}};

#[derive( PartialEq, Eq, Clone)]
enum Garden {
    Start,
    Plots,
    Rocks,
}

const P2_STEPS: usize = 26501365;

pub fn solve(input: &str) -> Solution {
    let original_grid: Grid<Garden> = Grid::load(input);
    let original_start_index = original_grid.locations.iter().position(|l| *l == Garden::Start).unwrap();
    let original_x = original_start_index % original_grid.cols;
    let original_y = original_start_index / original_grid.cols;
    let sz = original_grid.rows;
    let grid = original_grid.repeated(5, 5);

    let start_index = (original_y + 2 * sz) * grid.cols +  original_x + 2 * sz;

    let mut p1 = 0;

    let mut reached = vec![false; grid.locations.len()];
    reached[start_index] = true;
    let mut counts = vec![];

    let mut step = 0;
    loop {
        if step == 64 { p1 = reached.iter().filter(|r| **r).count() }
        if step % sz == P2_STEPS % sz {            
            counts.push(reached.iter().filter(|r| **r).count());
            if counts.len() == 3 {
                break;
            }
        }
        let mut v = vec![false; reached.len()];
        for (index,r) in reached.iter().enumerate() {
            if !r { continue; }
            if grid.locations[index - grid.cols] != Garden::Rocks { v[index - grid.cols] = true; }
            if grid.locations[index + grid.cols] != Garden::Rocks { v[index + grid.cols] = true; }
            if grid.locations[index - 1] != Garden::Rocks { v[index - 1] = true; }
            if grid.locations[index + 1] != Garden::Rocks { v[index + 1] = true; }
        }
        reached = v;
        step += 1;
    }
        
    let d = counts[0];
    let a = counts[2] - counts[1]*2 + d;
    let c = counts[1] - d - a/2;
        
    let n = P2_STEPS / sz;
    let p2 = a * n * n / 2 + c * n + d;

    Solution::new(p1, p2)
}

impl GridElement for Garden {
    fn from_char(c: &char) -> Option<Self> { 
        match c {
            '.' => Some(Self::Plots),
            '#' => Some(Self::Rocks),
            'S' => Some(Self::Start),
            _ => None,
        }
    }
    fn to_char(&self) -> char { 
        match self {
            Self::Plots => '.',
            Self::Rocks => '"',
            Self::Start => 'S',
        }
    }
}