// https://adventofcode.com/2021/day/20
use crate::common::Solution;
use itertools::Itertools;

const ORIGINAL_SIZE: usize = 100;
const MARGIN: usize = 60;
const SIZE: usize = ORIGINAL_SIZE + 2*MARGIN;

type Grid = [[u16;SIZE];SIZE];

fn display(grid: &Grid) {
    for row in grid.iter() {
        for c in row.iter() {
            print!("{}", if *c == 1 {'#'} else {'.'});
        }
        println!("");
    }
}

fn enhance(grid: &Grid, algorithm: &Vec<u16>) -> Grid {
    let mut output = [[0u16;SIZE];SIZE];
    for y in 1..(SIZE-1) {
        for x in 1..(SIZE-1) {
            let value =
            (grid[y-1][x-1] << 8) +
            (grid[y-1][x  ] << 7) +
            (grid[y-1][x+1] << 6) +
            (grid[y  ][x-1] << 5) +
            (grid[y  ][x  ] << 4) +
            (grid[y  ][x+1] << 3) +
            (grid[y+1][x-1] << 2) +
            (grid[y+1][x  ] << 1) +
            (grid[y+1][x+1]     );
            output[y][x] = algorithm[value as usize];
        }
    }
    let edge_value = output[2][2];
    for i in 0..SIZE {
        
        output[0][i] = edge_value;
        output[SIZE-1][i] = edge_value;
        output[i][0] = edge_value;
        output[i][SIZE-1] = edge_value;
    }
    output
}

fn count(grid: &Grid) -> usize {
    grid.iter().map(|row| row.iter().filter(|c| **c == 1).count()).sum()
}


pub fn solve(input: &str) -> Solution {
    
    let (image_enhancement_algorithm, input_image) = input.split_once("\n\n").unwrap();
    let image_enhancement_algorithm: Vec<u16> = image_enhancement_algorithm.as_bytes().iter().map(|b|
        match b { b'#' => 1, b'.' => 0, _ => panic!("Bad format") }).collect();


    let mut grid = [[0u16;SIZE];SIZE];

    for (y, line) in input_image.lines().enumerate() {
        for (x, b) in line.as_bytes().iter().enumerate() {
            grid[y+MARGIN][x+MARGIN] = match b { b'#' => 1, b'.' => 0, _ => panic!("Bad format") }
        }
    }
    for _ in 0..50 {
        grid = enhance(&grid, &image_enhancement_algorithm);
    }
    // display(&grid);


    println!("{}", count(&grid));


    let m1 = 0;
    let m2 = 0;
    // 5280 not right 

    Solution::new(m1,m2)
}
