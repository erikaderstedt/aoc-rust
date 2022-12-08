// https://adventofcode.com/2022/day/8

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let grid: Vec<Vec<u8>> = 
    input.lines().map(|line| line.chars().map(|c| (c as u8)-('0' as u8)).collect()).collect();
    let w = grid[0].len();
    let h = grid.len();

    let p1: usize = w*h - (w-2)*(h-2) + (1..(h-1)).map(|y0|
        (1..(w-1)).filter(|x0| {
            let tree_height = grid[y0][*x0];
            (0..*x0).find(|x| grid[y0][*x] >= tree_height).is_none() ||
            ((x0+1)..w).find(|x| grid[y0][*x] >= tree_height).is_none() ||
            (0..y0).find(|y| grid[*y][*x0] >= tree_height).is_none() ||
            ((y0+1)..h).find(|y| grid[*y][*x0] >= tree_height).is_none()
        }).count()
    ).sum::<usize>();

    let p2: usize = (1..(h-1)).map(|y0|
        (1..(w-1)).map(|x0| {
            let tree_height = grid[y0][x0];
            let up = (0..y0).rev().take_while(|y| grid[*y][x0] < tree_height).count();
            let down = ((y0+1)..h).take_while(|y| grid[*y][x0] < tree_height).count();
            let left = (0..x0).rev().take_while(|x| grid[y0][*x] < tree_height).count();
            let right = ((x0+1)..w).take_while(|x| grid[y0][*x] < tree_height).count();
            (if left == x0 { left } else { left  + 1 }) *
            (if up == y0 { up } else { up  + 1 }) *
            (if right == (w-1-x0) { right } else { right  + 1 }) *
            (if down == (h-1 - y0) { down } else { down  + 1 })

        }).max().unwrap()
    ).max().unwrap();

    Solution::new(p1,p2)
}

