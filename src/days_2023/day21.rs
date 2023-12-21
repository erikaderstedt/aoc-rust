// https://adventofcode.com/2023/day/21

use std::collections::HashSet;

use crate::{common::Solution, grid::{Grid, GridElement}};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Garden {
    Start,
    Plots,
    Rocks,
}

const MULT: usize = 2;

pub fn solve(input: &str) -> Solution {
    let grid: Grid<Garden> = Grid::load(input);

    let start_index = grid.locations.iter().position(|l| *l == Garden::Start).unwrap();
    // Step
    let p1 = {
        let mut grid: Grid<Garden> = grid.clone();
        grid.enclose(Garden::Rocks);

        // Instead of hashset, use vec![]
        let mut reached: HashSet<usize> = HashSet::new();
        reached.insert(grid.locations.iter().position(|l| *l == Garden::Start).unwrap());
    
        for _i in 0..64 {
            let mut v: Vec<usize> = vec![];
            for r in reached.iter() {
                v.push(r - grid.cols);
                v.push(r - 1);
                v.push(r + 1);
                v.push(r + grid.cols)            
            }
            reached = v.into_iter().filter(|p| grid.locations[*p] != Garden::Rocks).collect();
        }
        reached.len()
    };

    let mut v = vec![];
    let p2 = {
        let mut reached: HashSet<(usize,usize)> = HashSet::new();
        reached.insert((start_index % grid.cols + MULT * grid.cols,
            start_index / grid.cols + MULT * grid.rows));
    
        for i in 0..=(grid.rows*2 + 26501365 % grid.rows) {

            if i % grid.rows == 26501365 % grid.rows {
                v.push(reached.len());
            }

            let mut v: Vec<(usize,usize)> = vec![];
            for (x, y) in reached.into_iter() {
                v.push((x-1, y));
                v.push((x+1, y));
                v.push((x, y-1));
                v.push((x, y+1));
            }
            reached = v.into_iter()
                .filter(|(x,y)| {
                    let index = (x % grid.cols) + (y % grid.rows) * grid.cols;
                    grid.locations[index] != Garden::Rocks })
                .collect();

            
        }
        let d = v[0];
        let a = v[2] - v[1]*2 + d;
        let c = v[1] - d - a/2;
        
        let n = 26501365 / grid.rows;
        a * n * n / 2 + c * n + d
    };


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