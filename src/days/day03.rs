use crate::common::Solution;
use crate::grid::{Grid,GridElement,Position};

#[derive(Eq,PartialEq,Clone,Debug)]
enum ForestTile {
    Open,
    Tree,
}

impl GridElement for ForestTile {
    fn from_char(c: &char) -> Option<ForestTile> {
        match c {
            '.' => Some(ForestTile::Open),
            '#' => Some(ForestTile::Tree),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            ForestTile::Open => '.',
            ForestTile::Tree => '#',
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let grid: Grid<ForestTile> = Grid::load(input);
    let slopes = [(1,1),(3,1),(5,1),(7,1),(1,2)];

    let num_trees: Vec<usize> = slopes.iter().map(|(col_step,row_step)| -> usize {
        (0..grid.rows)
            .step_by(*row_step)
            .enumerate()
            .filter(|(col_index,row)| -> bool {
            grid[&Position { row: *row, column: (col_index * col_step) % grid.cols }] == ForestTile::Tree
        })
            .count()
    }).collect();
    
    let p2: usize = num_trees.iter().product();
    
    Solution { part_1: num_trees[1].to_string(), part_2: p2.to_string() }
}