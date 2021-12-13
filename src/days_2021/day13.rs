// https://adventofcode.com/2021/day/12
use crate::common::Solution;

#[derive(Debug)]
enum Fold {
    AlongX(usize),
    AlongY(usize),
}

const WIDTH: usize = 1306;
const HEIGHT: usize = 894;

fn fold(grid: &mut [bool;WIDTH*HEIGHT], sizes: (usize, usize), fold: &Fold) -> (usize, usize) {
    match fold {
        Fold::AlongY(y) => {
            for i in (y+1)..sizes.1 {
                for x in 0..sizes.0 {
                    grid[(y-(i-y))*WIDTH + x] = grid[(y-(i-y))*WIDTH + x] || grid[i*WIDTH + x];
                }
            }
            (sizes.0, *y)
        },
        Fold::AlongX(x) => {
            for i in (x+1)..sizes.0 {
                for y in 0..sizes.1 {                    
                    grid[y*WIDTH + (x-(i-x))] = grid[y*WIDTH + (x-(i-x))] || grid[y*WIDTH + i];
                }
            }
            (*x, sizes.1)
        }
    }
}

fn num_visible(grid: &[bool;WIDTH*HEIGHT], sizes: (usize, usize)) -> usize {
    let mut s = 0;
    for y in 0..sizes.1 {
        for x in 0..sizes.0 {
            if grid[y*WIDTH + x] { s += 1; }
        }
    }
    s
}

fn display(grid: &[bool;WIDTH*HEIGHT], sizes: (usize, usize)) {
    for y in 0..sizes.1 {
        for x in 0..sizes.0 {
            if grid[y*WIDTH + x] {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

pub fn solve(input: &str) -> Solution {
    let mut grid = [false;WIDTH*HEIGHT];
    for line in input.lines().take_while(|&line| line != "") {
        let (x,y) = line.split_once(',').unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        grid[y*WIDTH+x] = true;
    }

    let folds: Vec<Fold> = input.lines().skip_while(|line| !line.starts_with("fold")).map(|line| {
        let (_, n) = line.split_once('=').unwrap();
        let l = n.parse::<usize>().unwrap();
        match line.as_bytes()[11] {
            b'y' => Fold::AlongY(l),
            b'x' => Fold::AlongX(l),
            _ => panic!("Bad format"),
        }
    }).collect();

    let sz = fold(&mut grid, (WIDTH,HEIGHT), &folds[0]);
    let m1 = num_visible(&grid, sz);
    let _all_folded = folds.iter().skip(1).fold(sz, |sz, f| {
        fold(&mut grid, sz, f)
    });
    // display(&grid, all_folded);

    Solution::new(m1,"JGAJEFKU")
}


