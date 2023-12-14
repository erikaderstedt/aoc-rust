// https://adventofcode.com/2023/day/14

use crate::{common::Solution, grid::{Grid, GridElement}};

const SAMPLING_START: usize = 100;
const NUM_SAMPLES: usize = 90;

#[derive(Debug,Clone,PartialEq, Eq)]
enum Ground {
    Empty,
    Cube,
    Round,
}

pub fn solve(input: &str) -> Solution {
    let mut grid: Grid<Ground> = Grid::load(input);

    slide_north(&mut grid);

    let p1 = total_load_on_north_support_beams(&grid);
    
    // Finish cycle
    slide_west(&mut grid);
    slide_south(&mut grid);
    slide_east(&mut grid);

    let mut loads: Vec<usize> = Vec::with_capacity(NUM_SAMPLES);
    for i in 1..SAMPLING_START+NUM_SAMPLES {
        if i >= SAMPLING_START {
            loads.push(total_load_on_north_support_beams(& grid))
        }
        run_cycle(& mut grid);
    }

    // Find one value that only occurs twice.
    let cycle_length = loads.iter().find_map(|value|
    if loads.iter().filter(|v| *v == value).count() == 2 {
        let a: Vec<(usize, &usize)> = loads.iter().enumerate().filter(|v| v.1 == value).collect();
        Some(a[1].0 - a[0].0)
    } else {
        None
    }).unwrap();

    let p2 = loads.into_iter().enumerate()
        .find(|(index, _)| (1000000000 - (index + SAMPLING_START)) % cycle_length == 0).unwrap().1;

    Solution::new(p1, p2)
}

fn slide_north(grid: &mut Grid<Ground>) {
    for row in 1..grid.rows {
        for column in 0..grid.cols {
            if grid.locations[row*grid.cols + column] == Ground::Round {
                if let Some(new_row) = (0..row)
                    .rev()
                    .take_while(|r| grid.locations[r*grid.cols + column] == Ground::Empty)
                    .last() {
                    grid.locations[new_row*grid.cols + column] = Ground::Round;
                    grid.locations[row*grid.cols + column] = Ground::Empty;
                }
            }
        }
    }
}

fn slide_west(grid: &mut Grid<Ground>) {
    for row in 0..grid.rows {
        for column in 1..grid.cols {
            if grid.locations[row*grid.cols + column] == Ground::Round {
                if let Some(new_col) = (0..column).rev().take_while(|c| grid.locations[row*grid.cols + c] == Ground::Empty).last() {
                    grid.locations[row * grid.cols + new_col] = Ground::Round;
                    grid.locations[row*grid.cols + column] = Ground::Empty;
                }
            }
        }
    }
}

fn slide_south(grid: &mut Grid<Ground>) {
        for row in (0..grid.rows-1).rev() {
            for column in 0..grid.cols {
                if grid.locations[row*grid.cols + column] == Ground::Round {
                if let Some(new_row) = (row+1..grid.rows).take_while(|r| grid.locations[r*grid.cols + column] == Ground::Empty).last() {
                    grid.locations[new_row * grid.cols + column] = Ground::Round;
                    grid.locations[row*grid.cols + column] = Ground::Empty;
                }
            }
        }
    }
}

fn slide_east(grid: &mut Grid<Ground>) {
    for row in 0..grid.rows {
        for column in (0..grid.cols-1).rev() {
            if grid.locations[row*grid.cols + column] == Ground::Round {
                if let Some(new_col) = (column+1..grid.cols).take_while(|c| grid.locations[row*grid.cols + c] == Ground::Empty).last() {
                    grid.locations[row * grid.cols + new_col] = Ground::Round;
                    grid.locations[row*grid.cols + column] = Ground::Empty;
                }
            }
        }
    }
}

fn run_cycle(grid: &mut Grid<Ground>) {
    slide_north(grid);
    slide_west(grid);
    slide_south(grid);
    slide_east(grid);
}

fn total_load_on_north_support_beams(grid: &Grid<Ground>) -> usize {
    (0..grid.rows)
        .rev()
        .map(|r| (grid.rows - r) * 
                  grid.locations[r*grid.cols..(r+1)*grid.cols].iter()
                    .filter(|l| **l == Ground::Round)
                    .count())
        .sum()
}

impl GridElement for Ground {
    fn from_char(c: &char) -> Option<Self> { 
        match c {
            '.' => Some(Self::Empty),
            '#' => Some(Self::Cube),
            'O' => Some(Self::Round),
            _ => None,
        }
    }
    fn to_char(&self) -> char { 
        match self {
            Self::Empty => '.',
            Self::Cube => '#',
            Self::Round => 'O',
        }
    }
}