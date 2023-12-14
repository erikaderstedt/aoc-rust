// https://adventofcode.com/2023/day/13

use crate::{common::Solution, grid::{Grid, GridElement}};

#[derive(Debug,Clone,PartialEq, Eq)]
enum Ground {
    Ash,
    Rock,
}

impl GridElement for Ground {
    fn from_char(c: &char) -> Option<Self> { 
        match c {
            '.' => Some(Self::Ash),
            '#' => Some(Self::Rock),
            _ => None,
        }
    }
    fn to_char(&self) -> char { 
        match self {
            Self::Ash => '.',
            Self::Rock => '#',
        }
    }
}

fn find_reflection(grid: &Grid<Ground>) -> Option<usize> {
    (0..grid.rows-1).find(|r| iterate_by_row(grid, *r).all(|x| x)).map(|r| 100 * (r + 1))
    .or_else(|| (0..grid.cols-1).find(|c| iterate_by_column(grid, *c).all(|x| x)).map(|c| c + 1))
}

fn find_reflection_with_smudge(grid: &Grid<Ground>) -> Option<usize> {
    (0..grid.rows).find(|r| iterate_by_row(grid, *r)
    .scan(0, |acc,x| 
        if *acc > 1  { None } else if !x { *acc += 1; Some(*acc) } else { Some(*acc) }).last() == Some(1))        
        .map(|r| 100 * (r + 1))
    .or_else(|| (0..grid.cols).find(|c| iterate_by_column(grid, *c)
    .scan(0, |acc,x| 
        if *acc > 1  { None } else if !x { *acc += 1; Some(*acc) } else { Some(*acc) }).last() == Some(1))        
    .map(|c| c + 1))
}

// TODO: replace Grid with a u32 for each row. 
// Use xor and count_ones to identify one row that differs by 1



pub fn solve(input: &str) -> Solution {
    let (p1, p2) = input.split("\n\n").map(|s| {        
        let r: Grid<Ground> = Grid::load(s);
        (find_reflection(&r).unwrap_or(0),
        find_reflection_with_smudge(&r).unwrap_or(0))
    }).fold((0,0), |acc, v| (acc.0 + v.0, acc.1 + v.1));

    Solution::new(p1, p2)
}

struct RowIterator<'a> {
    grid: &'a Grid<Ground>,
    index: usize,
    beginning: bool,
    other_index: usize,
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        
        if self.beginning || self.other_index >= self.grid.cols * self.grid.rows {
            None
        } else {
            let v = self.grid.locations[self.index] == self.grid.locations[self.other_index];
            self.index += 1;
            self.other_index += 1;
            if self.index % self.grid.cols == 0 {
                if self.index == self.grid.cols {
                    self.beginning = true;
                } else {
                    self.index -= self.grid.cols * 2;                        
                }
            }
            Some(v)
        }
    }
}

struct ColumnIterator<'a> {
    grid: &'a Grid<Ground>,
    index: usize,
    beginning: bool,
    other_index: usize,
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        
        if self.beginning || self.other_index >= self.grid.cols * self.grid.rows {
            None
        } else {
            let v = self.grid.locations[self.index] == self.grid.locations[self.other_index];
            if self.index == 0 {
                self.beginning = true;
            } else if self.index / self.grid.cols == 0 {
                self.other_index += self.grid.cols * (self.grid.rows - 1) + 1;
                self.index += self.grid.cols * (self.grid.rows - 1) - 1;
            } else {
                self.other_index -= self.grid.cols;
                self.index -= self.grid.cols;
            }
            Some(v)
        }
    }
}

fn iterate_by_row(grid: &Grid<Ground>, row: usize) -> RowIterator {
    RowIterator { grid, index: row * grid.cols, other_index: (row+1) * grid.cols, beginning: false }
}

fn iterate_by_column(grid: &Grid<Ground>, column: usize) -> ColumnIterator {
    ColumnIterator { grid, index: (grid.rows - 1) * grid.cols + column, 
        other_index: (grid.rows - 1) * grid.cols + column + 1, beginning: false }
}
