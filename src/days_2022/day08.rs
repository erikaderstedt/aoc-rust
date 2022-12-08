// https://adventofcode.com/2022/day/8

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let w = grid[0].len();
    let h = grid.len();

    let p1: usize = w*h - (w-2)*(h-2) + (1..(h-1)).map(|y0|
        (1..(w-1)).filter(|&x0| {
            let tree_height = grid[y0][x0];
            (0..x0).all(|x| grid[y0][x] < tree_height) ||
            ((x0+1)..w).all(|x| grid[y0][x] < tree_height) ||
            (0..y0).all(|y| grid[y][x0] < tree_height) ||
            ((y0+1)..h).all(|y| grid[y][x0] < tree_height)
        }).count()
    ).sum::<usize>();

    let p2: usize = (1..(h-1)).map(|y0|
        (1..(w-1)).map(|x0| {
            let tree_height = grid[y0][x0];
            (match ((y0+1)..h).find(|&y| grid[y][x0] >= tree_height) { Some(y) => y-y0, None => h-y0-1 }) *
            (match (1..y0).find(|&y| grid[y0-y][x0] >= tree_height) { Some(y) => y, None => y0 }) *
            (match (1..x0).find(|&x| grid[y0][x0-x] >= tree_height) { Some(x) => x, None => x0 }) *
            (match ((x0+1)..w).find(|&x| grid[y0][x] >= tree_height) { Some(x) => x-x0, None => w-x0-1 })
        }).max().unwrap_or(0)
    ).max().unwrap_or(0);

    Solution::new(p1,p2)
}

