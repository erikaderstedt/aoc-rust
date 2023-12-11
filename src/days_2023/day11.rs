// https://adventofcode.com/2023/day/11

use crate::{common::Solution, grid::{Grid, GridElement}};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Space {
    Empty,
    Galaxy,
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

pub fn solve(input: &str) -> Solution {
    let all_space: Grid<Space> = Grid::load(input);

    let cols_with_no_galaxies: Vec<usize> = (0..all_space.cols)
        .filter(|c| all_space.locations.iter()
            .skip(c - 1)
            .step_by(all_space.cols)
            .all(|v| *v == Space::Empty))
        .collect();

    let rows_with_no_galaxies: Vec<usize> = (0..all_space.cols)
        .filter(|r| all_space.locations.iter()
            .skip(r * all_space.cols)
            .take(all_space.cols)
            .all(|v| *v == Space::Empty))
        .collect();

    let factor = 1000000;

    let galaxies: Vec<usize> = all_space.locations.iter()
        .enumerate()
        .filter_map(|(index, v)| if *v == Space::Galaxy { Some(index) } else { None })
        .collect();

    // TODO: separate the empty rows and get a tuple out, so that 
    // the same calculation can be used for both p1 and p2.

    // TODO: store the galaxies by (row, column) instead, or maybe in addition to index, 
    // to skip all those extra division steps
    let p2: usize = galaxies.iter()
        .map(|index| -> usize {

            // Get distances to later galaxies    
            galaxies.iter()
                .filter(|other_index| **other_index > *index)
                .map(|other_index| {
                    
                    let r1 = index / all_space.cols;
                    let r2 = other_index / all_space.cols;

                    let c1 = index % all_space.cols;
                    let c2 = other_index % all_space.cols;

                    let row_steps = r2 - r1 + 
                        (r1..=r2)
                            .skip(1)
                            .filter(|r| rows_with_no_galaxies.contains(r))
                            .count() * (factor-1);

                    let col_steps = 
                    if c2 < c1 {
                        c1 - c2 + 
                        (c2..=c1)
                            .skip(1)
                            .filter(|c| cols_with_no_galaxies.contains(c))
                            .count() * (factor-1)

                    } else {
                        c2 - c1 + 
                        (c1..=c2)
                            .skip(1)
                            .filter(|c| cols_with_no_galaxies.contains(c))
                            .count() * (factor-1)

                    };

                    row_steps + col_steps                
                })
                .sum()
            })
        .sum();
   
    Solution::new(0, p2)
}
