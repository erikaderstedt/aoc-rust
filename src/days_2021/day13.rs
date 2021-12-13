// https://adventofcode.com/2021/day/12
use crate::common::Solution;
use std::collections::HashSet;

enum Fold {
    AlongX(usize),
    AlongY(usize),
}

const WIDTH: usize = 40;
const HEIGHT: usize = 7;

fn ocr(grid: &[[bool;40];HEIGHT], x: usize) -> char {
    if grid[1][x+3] {
        if grid[3][x+1] {
            if grid[0][x+3] {
                if grid[2][x+1] { 'X' } 
                else if grid[4][x+3] { 'W' } else { 'Z' }
            } 
            else if grid[4][x+2] { 'R' } 
            else if grid[0][x+0] { 'P' } else { 'A' }
        }
        else if grid[0][x+0] {
            if grid[0][x+3] {
				if !grid[5][x+0] {
					if grid[4][x+1] { 'V' } else { 'U' }
				} 
                else if grid[1][x+2] { 'M'} 
                else if grid[2][x+2] { 'H' } else { 'N' }
			} 
            else if grid[2][x+1] { 'B' } else { 'D' }
        }
		else if grid[4][x+1] { 'Q' }
		else if grid[5][x+3] { 'G' }
		else if grid[0][x+3] { 'J' }
		else if grid[2][x+3] { 'O' }
		else if grid[2][x+0] { 'C' } else  { 'S' }
    }
    else if grid[5][x+3] {
		if grid[2][x+2] { 'E' }	
        else if grid[0][x+3] { 'K' } else { 'L' }
    }
	else if !grid[0][x+1] { 'Y' }
	else if grid[5][x+2] { 'I' }
    else if grid[0][x+3] { 'F' } else { 'T' }
}

const ORIGINAL_X_SIZE: usize = 1306;
const ORIGINAL_Y_SIZE: usize = 894;
fn create_coordinate_maps<'a,I>(folds: I) -> (Vec<usize>, Vec<usize>) 
where I: Iterator<Item=&'a Fold> {
    let mut x_map = Vec::with_capacity(ORIGINAL_X_SIZE);
    let mut y_map = Vec::with_capacity(ORIGINAL_Y_SIZE);
    for i in 0..ORIGINAL_X_SIZE { 
        x_map.push(i);
        y_map.push(i);
    }
    for i in 0..ORIGINAL_Y_SIZE { 
        y_map.push(i);
    }
    let mut x_size = ORIGINAL_X_SIZE;
    let mut y_size = ORIGINAL_Y_SIZE;
    for fold in folds {
        match fold {
            Fold::AlongY(y) => {
                y_map[*y] = 0;
                for i in (y+1)..y_size {
                    y_map[i] = 2*y - i;
                }
                for i in y_size..ORIGINAL_Y_SIZE {
                    y_map[i] = y_map[y_map[i]];
                }
                y_size = *y;
            },
            Fold::AlongX(x) => {
                x_map[*x] = 0;
                for i in (x+1)..x_size {
                    x_map[i] = 2*x - i;
                }
                for i in x_size..ORIGINAL_X_SIZE {
                    x_map[i] = x_map[x_map[i]];
                }
                x_size = *x;
            },
        }
    }
    (x_map, y_map)
}
pub fn solve(input: &str) -> Solution {

    let folds: Vec<Fold> = input.lines().filter_map(|line| {
        match line.split_once('=') {
            Some((_,b)) => {
                let l = b.parse::<usize>().unwrap();
                match line.as_bytes()[11] {
                    b'y' => Some(Fold::AlongY(l)),
                    b'x' => Some(Fold::AlongX(l)),
                    _ => panic!("Bad format"),
                }
            },
            None => None,
        }
    }).collect();

    let num_original_points = input.lines().count() - 1 - folds.len();
    let grid_points: Vec<(usize,usize)> = input.lines().take(num_original_points).map(|line| {
        let (x,y) = line.split_once(',').unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    }).collect();

    // For pt 1, use HashSet of coordinates because the grid will be too large and slow.
    let m1 = {
        let (x_map, y_map) = create_coordinate_maps(folds.iter().take(1));
        let points: HashSet<(usize,usize)> = grid_points.iter().map(|(x,y)| {
            (x_map[*x], y_map[*y])
        }).collect();
        points.len()
    };

    let m2 = {
        // For pt 2, use a grid. Grid will be the size of the last folds in each direction
        // For now, hard code these
        let (x_map, y_map) = create_coordinate_maps(folds.iter());
        let mut grid = [[false;WIDTH];HEIGHT];
        for (x0, y0) in grid_points.into_iter() {
            grid[y_map[y0]][x_map[x0]] = true;
        }
        (0..8).map(|i| ocr(&grid, i*5)).collect::<String>()
    };

    Solution::new(m1,m2)
}


