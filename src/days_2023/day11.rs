// https://adventofcode.com/2023/day/11

use crate::{common::Solution, grid::{Grid, GridElement}};

#[derive(PartialEq, Eq, Clone)]
enum Space {
    Empty,
    Galaxy,
}

pub fn solve(input: &str) -> Solution {
    let all_space: Grid<Space> = Grid::load(input);

    let empty_cols: Vec<usize> = (0..all_space.cols)
        .scan(0, |acc, c| {
            if all_space.locations.iter()
                .skip(c)
                .step_by(all_space.cols)
                .all(|v| *v == Space::Empty) {
                    *acc += 1;
            }
            Some(*acc) })
        .collect();

    let empty_rows: Vec<usize> = (0..all_space.rows)
        .scan(0, |acc, r| {
            if all_space.locations.iter()
                .skip(r * all_space.cols)
                .take(all_space.cols)
                .all(|v| *v == Space::Empty) {
                    *acc += 1;
            }
            Some(*acc) })
        .collect();

    let galaxies: Vec<(usize,usize)> = all_space.locations.iter()
        .enumerate()
        .filter(|(_, v)| **v == Space::Galaxy)
        .map(|(index, _)| (index / all_space.cols, index % all_space.cols))
        .collect();
        
    let sum_list_of_tuples = |(a0,b0): (usize,usize), (a,b): (usize,usize)| -> (usize,usize) { (a0 + a, b0 + b) };

    let (distance, expansion) = galaxies.iter()
        .enumerate()
        .map(|(index, (row, col))| galaxies.iter()
                                .skip(index + 1)
                                .map(|(other_row, other_col)|                                 
                                    (other_row - row + other_col.abs_diff(*col),
                                    empty_rows[*other_row] - empty_rows[*row] + empty_cols[*other_col].abs_diff(empty_cols[*col])))
                                .fold((0,0), sum_list_of_tuples))
        .fold((0,0), sum_list_of_tuples);

    let p1 = distance + 1 * expansion;
    let p2 = distance + 999999 * expansion;
   
    Solution::new(p1, p2)
}

impl GridElement for Space {
    fn from_char(c: &char) -> Option<Self> { 
        match c {
            '.' => Some(Self::Empty),
            '#' => Some(Self::Galaxy),
            _ => None,
        }
    }
    fn to_char(&self) -> char { 
        match self {
            Self::Empty => '.',
            Self::Galaxy => '#',
        }
    }
}