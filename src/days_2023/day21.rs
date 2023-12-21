// https://adventofcode.com/2023/day/21

use crate::{common::Solution, grid::{Grid, GridElement}};

#[derive( PartialEq, Eq, Clone)]
enum Garden {
    Start,
    Plots,
    Rocks,
}

const P1_STEPS: usize = 64;
const P2_STEPS: usize = 26501365;

pub fn solve(input: &str) -> Solution {
    let original_grid: Grid<Garden> = Grid::load(input);
    let original_start_index = original_grid.locations.iter().position(|l| *l == Garden::Start).unwrap();
    let original_x = original_start_index % original_grid.cols;
    let original_y = original_start_index / original_grid.cols;
    
    // Repeat grid 5x5. Since the elf reaches at most 1 step out per iteration, 
    // and we need 2x131 + 65 steps this is enough.
    let sz = original_grid.rows;
    let grid = original_grid.repeated(5, 5);
    let start_index = (original_y + 2 * sz) * grid.cols +  original_x + 2 * sz;

    let mut p1 = 0;
    let mut reached = vec![false; grid.locations.len()];
    reached[start_index] = true;
    let mut counts = vec![];

    let mut step = 0;
    loop {
        let num_visited = || reached.iter()
            .enumerate()
            .filter(|(index, r)| grid.locations[*index] != Garden::Rocks && **r)
            .count();

        if step == P1_STEPS { p1 = num_visited() }
        if step % sz == P2_STEPS % sz {            
            counts.push(num_visited());
            if counts.len() == 3 {
                break;
            }
        }
        let mut v = vec![false; reached.len()];
        for (index,_) in reached.iter()
            .enumerate()
            .filter(|(index, r)| **r && grid.locations[*index] != Garden::Rocks) {
            v[index - grid.cols] = true;
            v[index + grid.cols] = true;
            v[index - 1] = true;
            v[index + 1] = true;
        }
        reached = v;
        step += 1;
    }
    
    // counts[n] = A * n^2 / 2 + C * n + D
    // counts[0] = 0 + 0 + D
    // counts[1] = A/2 + C + D
    // counts[2] = A * 2 + 2 * C + D
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