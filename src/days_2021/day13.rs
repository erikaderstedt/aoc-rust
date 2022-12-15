// https://adventofcode.com/2021/day/12
use crate::common::{Solution,read_5x6_characters_off_grid};
use std::collections::HashSet;

enum Fold {
    AlongX(usize),
    AlongY(usize),
}

fn final_folded_index(i: usize, size: usize) -> usize {
    let n = (i + size + 1)/(size*2+2);
    if i < (size+1)*2*n {
        (size+1)*2*n-2-i
    } else {
        i - (size+1)*2*n
    }
}

pub fn solve(input: &str) -> Solution {
    // Size of grid is first x fold * 2 + 1, and first y fold * 2 + 1
    // folds are always in the middle.
    let folds: Vec<Fold> = input.lines().skip_while(|line| *line != "").filter_map(|line| {
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
        let points: HashSet<(usize,usize)> = match folds[0] {
        Fold::AlongX(x0) => grid_points.iter().map(|(x,y)| {
                                    (if *x < x0 { *x } else { 2*x0 - x }, *y)
                            }).collect(),
        Fold::AlongY(y0) => grid_points.iter().map(|(x,y)| {
                                    (*x, if *y < y0 { *y } else { 2*y0 - y })
                            }).collect()
                        };
        points.len()
    };

    let m2 = {
        // For pt 2, use a grid. Grid will be the size of the last folds in each direction
        let final_x_width = folds.iter()
            .filter_map(|f| match f {
                Fold::AlongX(x) => Some(*x),
                _ => None
            }).last().unwrap();
        let final_y_width = folds.iter()
            .filter_map(|f| match f {
                Fold::AlongY(y) => Some(*y),
                _ => None
            }).last().unwrap();

        let mut grid = vec![vec![false;final_x_width];final_y_width];
        for (x, y) in grid_points.into_iter() {
            grid[final_folded_index(y, final_y_width)][final_folded_index(x, final_x_width)] = true;
        }
        match read_5x6_characters_off_grid(&grid.iter().flatten().cloned().collect()) {
            Ok(s) => s,
            Err(a) => {
                println!("Error: {}", a);
                for line in grid.into_iter() {
                    for c in line {
                        print!("{}", if c { '█' } else { ' ' });
                    }
                    println!("");
                }
                "?????".to_string()
            }
        }
    };

    Solution::new(m1,m2)
}
